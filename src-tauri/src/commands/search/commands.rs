use rusqlite::params;
use std::time::SystemTime;

use super::db::{ensure_state, system_time_to_epoch, INDEX};
use super::engine::{
    append_filter_conditions, fuzzy_score, get_frecency_score, path_rank_boost,
    search_content_parallel,
};
use super::types::{IndexStatus, SearchHistoryEntry, SearchQuery, SearchResult};
use super::types::{parse_smart_query, type_to_extensions};

// ---------------------------------------------------------------------------
// search_files — main search command (Layers 1 + 2 + 3)
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn search_files(query: SearchQuery) -> Result<Vec<SearchResult>, String> {
    ensure_state();
    let guard = match INDEX.try_lock() {
        Ok(g) => g,
        Err(_) => return Ok(vec![]),
    };
    let state = guard.as_ref().unwrap();

    let limit = query.limit.unwrap_or(50).min(200);
    let raw_query = query.query.clone();

    let mut pq = parse_smart_query(&query.query);

    if let Some(ref scope) = query.scope {
        if !scope.is_empty() {
            pq.scope = Some(scope.clone());
        }
    }
    if let Some(ref exts) = query.extensions {
        if !exts.is_empty() {
            pq.extensions.extend(exts.iter().map(|e| e.to_lowercase()));
        }
    }
    if let Some(min) = query.min_size {
        pq.min_size = pq.min_size.or(Some(min));
    }
    if let Some(max) = query.max_size {
        pq.max_size = pq.max_size.or(Some(max));
    }

    if let Some(ref type_name) = pq.type_filter.clone() {
        let type_exts = type_to_extensions(type_name);
        if !type_exts.is_empty() {
            pq.extensions.extend(type_exts.iter().map(|e| e.to_string()));
        }
    }

    let has_name_terms = !pq.name_terms.is_empty();
    let search_lower: Vec<String> = pq.name_terms.iter().map(|t| t.to_lowercase()).collect();

    let mut results: Vec<SearchResult> = Vec::new();

    if has_name_terms {
        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        for term in &search_lower {
            conditions.push("name_lower LIKE ?".to_string());
            param_values.push(Box::new(format!("%{}%", term)));
        }
        append_filter_conditions(&pq, &mut conditions, &mut param_values, "");

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT path, name, is_dir, size, modified, extension FROM files {} LIMIT ?",
            where_clause
        );
        param_values.push(Box::new((limit * 20) as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();

        let mut scored: Vec<(SearchResult, i32)> = Vec::new();

        if let Ok(mut stmt) = state.db.prepare(&sql) {
            if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i32>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, f64>(4)?,
                    row.get::<_, String>(5)?,
                ))
            }) {
                for row in rows.filter_map(|r| r.ok()) {
                    let (path, name, is_dir, size, modified, extension) = row;

                    let mut total_score: i32 = 0;
                    let mut all_match = true;
                    for term in &search_lower {
                        if let Some((score, _)) = fuzzy_score(term, &name) {
                            total_score += score;
                        } else {
                            all_match = false;
                            break;
                        }
                    }

                    if all_match {
                        let name_lower = name.to_lowercase();

                        let name_stem = if let Some(dot) = name_lower.rfind('.') {
                            &name_lower[..dot]
                        } else {
                            &name_lower
                        };
                        if search_lower.len() == 1 && name_stem == search_lower[0] {
                            total_score += 50;
                        } else if search_lower.len() == 1 && name_lower == search_lower[0] {
                            total_score += 50;
                        }

                        if search_lower.len() == 1 && name_lower.starts_with(&search_lower[0]) {
                            total_score += 20;
                        }

                        total_score += path_rank_boost(&path);
                        total_score += get_frecency_score(&state.db, &path);

                        scored.push((
                            SearchResult {
                                path,
                                name,
                                is_dir: is_dir != 0,
                                size: size as u64,
                                modified,
                                extension,
                                rank: total_score as f64,
                                content_snippet: None,
                            },
                            total_score,
                        ));
                    }
                }
            }
        }

        scored.sort_by(|a, b| b.1.cmp(&a.1));
        results = scored.into_iter().take(limit as usize).map(|(r, _)| r).collect();
    } else {
        let mut conditions: Vec<String> = Vec::new();
        let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
        append_filter_conditions(&pq, &mut conditions, &mut param_values, "");

        let where_clause = if conditions.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", conditions.join(" AND "))
        };

        let sql = format!(
            "SELECT path, name, is_dir, size, modified, extension FROM files {} ORDER BY name_lower LIMIT ?",
            where_clause
        );
        param_values.push(Box::new(limit as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            param_values.iter().map(|p| p.as_ref()).collect();

        if let Ok(mut stmt) = state.db.prepare(&sql) {
            if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                Ok(SearchResult {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    is_dir: row.get::<_, i32>(2)? != 0,
                    size: row.get::<_, i64>(3)? as u64,
                    modified: row.get(4)?,
                    extension: row.get(5)?,
                    rank: 0.0,
                    content_snippet: None,
                })
            }) {
                results.extend(rows.filter_map(|r| r.ok()));
            }
        }
    }

    if has_name_terms && pq.search_content && results.len() < 5 && limit >= 50 {
        let content_limit = 10;
        let content_results = search_content_parallel(&search_lower, content_limit);
        let existing_paths: std::collections::HashSet<String> =
            results.iter().map(|r| r.path.clone()).collect();
        for r in content_results {
            if !existing_paths.contains(&r.path) {
                results.push(r);
            }
        }
    }

    let result_count = results.len() as u32;
    if raw_query.trim().len() >= 3 {
        let now = system_time_to_epoch(SystemTime::now());
        state.db.execute(
            "INSERT INTO search_history (query, timestamp, result_count) VALUES (?1, ?2, ?3)",
            params![raw_query, now, result_count],
        ).ok();
    }

    Ok(results)
}

// ---------------------------------------------------------------------------
// Status & history commands
// ---------------------------------------------------------------------------

#[tauri::command]
pub fn get_index_status() -> Result<IndexStatus, String> {
    ensure_state();
    let guard = match INDEX.try_lock() {
        Ok(g) => g,
        Err(_) => {
            return Ok(IndexStatus {
                indexing: true,
                total_files: 0,
                indexed_paths: vec![],
                last_updated: None,
                content_indexed: 0,
            });
        }
    };
    let state = guard.as_ref().unwrap();

    Ok(IndexStatus {
        indexing: state.indexing,
        total_files: state.total_files,
        indexed_paths: state.indexed_paths.clone(),
        last_updated: state.last_updated,
        content_indexed: 0,
    })
}

#[tauri::command]
pub fn clear_index() -> Result<(), String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    state
        .db
        .execute_batch(
            "DELETE FROM files;
             DELETE FROM file_content;
             DELETE FROM trigrams;
             DELETE FROM index_meta;",
        )
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn record_file_open(path: String) -> Result<(), String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    let now = system_time_to_epoch(SystemTime::now());
    state.db.execute(
        "INSERT INTO frecency (path, open_count, last_opened) VALUES (?1, 1, ?2)
         ON CONFLICT(path) DO UPDATE SET open_count = open_count + 1, last_opened = ?2",
        params![path, now],
    ).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_search_history() -> Result<Vec<SearchHistoryEntry>, String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();

    let mut entries = Vec::new();
    if let Ok(mut stmt) = state.db.prepare(
        "SELECT DISTINCT query, MAX(timestamp) as ts, MAX(result_count) as rc
         FROM search_history
         GROUP BY query
         ORDER BY ts DESC
         LIMIT 20"
    ) {
        if let Ok(rows) = stmt.query_map([], |row| {
            Ok(SearchHistoryEntry {
                query: row.get(0)?,
                timestamp: row.get(1)?,
                result_count: row.get::<_, u32>(2)?,
            })
        }) {
            entries.extend(rows.filter_map(|r| r.ok()));
        }
    }
    Ok(entries)
}

#[tauri::command]
pub fn clear_search_history() -> Result<(), String> {
    ensure_state();
    let guard = INDEX.lock().unwrap();
    let state = guard.as_ref().unwrap();
    state.db.execute("DELETE FROM search_history", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}
