use rusqlite::{params, Connection};
use std::thread;

use super::db::{db_path, system_time_to_epoch};
use super::types::{ParsedQuery, SearchResult};

// ---------------------------------------------------------------------------
// Fuzzy matching (fzf-style)
// ---------------------------------------------------------------------------

pub(crate) fn fuzzy_score(pattern: &str, target: &str) -> Option<(i32, Vec<usize>)> {
    let pattern_lower: Vec<char> = pattern.to_lowercase().chars().collect();
    let target_lower: Vec<char> = target.to_lowercase().chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    if pattern_lower.is_empty() {
        return Some((0, vec![]));
    }
    if pattern_lower.len() > target_lower.len() {
        return None;
    }

    let mut indices = Vec::with_capacity(pattern_lower.len());
    let mut ti = 0;
    for &pc in &pattern_lower {
        let mut found = false;
        while ti < target_lower.len() {
            if target_lower[ti] == pc {
                indices.push(ti);
                ti += 1;
                found = true;
                break;
            }
            ti += 1;
        }
        if !found {
            return None;
        }
    }

    let mut score: i32 = 0;

    if indices[0] == 0 {
        score += 10;
    }

    for i in 1..indices.len() {
        if indices[i] == indices[i - 1] + 1 {
            score += 8;
        }
    }

    for &idx in &indices {
        if idx == 0 {
            score += 5;
        } else {
            let prev = target_chars[idx - 1];
            if prev == '_' || prev == '-' || prev == '.' || prev == ' ' || prev == '\\' || prev == '/' {
                score += 5;
            }
            if target_chars[idx].is_uppercase() && idx > 0 && target_chars[idx - 1].is_lowercase() {
                score += 3;
            }
        }
    }

    score -= (target_lower.len() as i32 - pattern_lower.len() as i32) / 3;

    let spread = indices.last().unwrap_or(&0) - indices[0];
    score -= (spread as i32 - pattern_lower.len() as i32) / 2;

    Some((score, indices))
}

// ---------------------------------------------------------------------------
// Path-based ranking boost / penalty
// ---------------------------------------------------------------------------

pub(crate) fn path_rank_boost(path: &str) -> i32 {
    let path_lower = path.to_lowercase();

    let system_patterns = [
        "\\appdata\\local\\",
        "\\appdata\\roaming\\",
        "\\appdata\\locallow\\",
        "\\programdata\\",
        "\\program files\\",
        "\\program files (x86)\\",
        "\\windows\\",
        "\\windows.old\\",
        "\\$recycle.bin\\",
        "\\.cargo\\",
        "\\.rustup\\",
        "\\.nuget\\",
        "\\.npm\\",
        "\\.cache\\",
        "\\.vscode\\",
        "\\node_modules\\",
        "\\__pycache__\\",
        "\\.git\\",
        "\\assembly\\",
        "\\winsxs\\",
    ];

    for pat in &system_patterns {
        if path_lower.contains(pat) {
            return -40;
        }
    }

    let user_patterns = [
        ("\\desktop\\", 25),
        ("\\documents\\", 20),
        ("\\downloads\\", 20),
        ("\\pictures\\", 15),
        ("\\videos\\", 15),
        ("\\music\\", 15),
        ("\\onedrive\\", 10),
        ("\\projects\\", 15),
        ("\\github\\", 15),
        ("\\source\\", 10),
        ("\\repos\\", 10),
    ];

    for (pat, boost) in &user_patterns {
        if path_lower.contains(pat) {
            return *boost;
        }
    }

    let depth = path.matches('\\').count();
    if depth > 8 {
        return -10;
    } else if depth > 6 {
        return -5;
    }

    0
}

// ---------------------------------------------------------------------------
// Frecency scoring
// ---------------------------------------------------------------------------

pub(crate) fn get_frecency_score(conn: &Connection, path: &str) -> i32 {
    if let Ok(mut stmt) = conn.prepare(
        "SELECT open_count, last_opened FROM frecency WHERE path = ?1"
    ) {
        if let Ok((count, last_opened)) = stmt.query_row(params![path], |row| {
            Ok((row.get::<_, i32>(0)?, row.get::<_, f64>(1)?))
        }) {
            let now = system_time_to_epoch(std::time::SystemTime::now());
            let age_hours = (now - last_opened) / 3600.0;

            let recency_weight = if age_hours < 1.0 {
                10.0
            } else if age_hours < 24.0 {
                5.0
            } else if age_hours < 168.0 {
                2.0
            } else {
                1.0
            };

            return (count as f64 * recency_weight) as i32;
        }
    }
    0
}

// ---------------------------------------------------------------------------
// Content search (Layer 3) — parallel thread
// ---------------------------------------------------------------------------

pub(crate) fn search_content_parallel(search_terms: &[String], limit: usize) -> Vec<SearchResult> {
    let db_path_val = db_path();
    let terms = search_terms.to_vec();

    let handle = thread::spawn(move || {
        let conn = match Connection::open(&db_path_val) {
            Ok(c) => c,
            Err(_) => return vec![],
        };

        let mut content_conditions: Vec<String> = Vec::new();
        let mut content_params: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

        for term in &terms {
            content_conditions.push("c.content_lower LIKE ?".to_string());
            content_params.push(Box::new(format!("%{}%", term)));
        }

        let content_sql = format!(
            "SELECT f.path, f.name, f.is_dir, f.size, f.modified, f.extension, c.content_lower
             FROM file_content c
             JOIN files f ON f.path = c.path
             WHERE {}
             LIMIT ?",
            content_conditions.join(" AND ")
        );
        content_params.push(Box::new(limit as i64));

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            content_params.iter().map(|p| p.as_ref()).collect();

        let mut results = Vec::new();
        if let Ok(mut stmt) = conn.prepare(&content_sql) {
            if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| {
                let content_text: String = row.get(6)?;
                let snippet = extract_snippet(&content_text, &terms[0]);
                Ok(SearchResult {
                    path: row.get(0)?,
                    name: row.get(1)?,
                    is_dir: row.get::<_, i32>(2)? != 0,
                    size: row.get::<_, i64>(3)? as u64,
                    modified: row.get(4)?,
                    extension: row.get(5)?,
                    rank: -100.0,
                    content_snippet: Some(snippet),
                })
            }) {
                results.extend(rows.filter_map(|r| r.ok()));
            }
        }
        results
    });

    handle.join().unwrap_or_default()
}

// ---------------------------------------------------------------------------
// Filter condition builder
// ---------------------------------------------------------------------------

pub(crate) fn append_filter_conditions(
    pq: &ParsedQuery,
    conditions: &mut Vec<String>,
    params: &mut Vec<Box<dyn rusqlite::types::ToSql>>,
    prefix: &str,
) {
    if let Some(ref scope) = pq.scope {
        if !scope.is_empty() {
            conditions.push(format!("{}path LIKE ?", prefix));
            params.push(Box::new(format!("{}%", scope)));
        }
    }
    if !pq.extensions.is_empty() {
        let placeholders: Vec<String> = pq.extensions.iter().map(|_| "?".to_string()).collect();
        conditions.push(format!("{}extension IN ({})", prefix, placeholders.join(",")));
        for ext in &pq.extensions {
            params.push(Box::new(ext.clone()));
        }
    }
    if let Some(min) = pq.min_size {
        conditions.push(format!("{}size >= ?", prefix));
        params.push(Box::new(min as i64));
    }
    if let Some(max) = pq.max_size {
        conditions.push(format!("{}size <= ?", prefix));
        params.push(Box::new(max as i64));
    }
    if let Some(after) = pq.modified_after {
        conditions.push(format!("{}modified >= ?", prefix));
        params.push(Box::new(after));
    }
    if let Some(before) = pq.modified_before {
        conditions.push(format!("{}modified <= ?", prefix));
        params.push(Box::new(before));
    }
    if let Some(after) = pq.created_after {
        conditions.push(format!("{}created >= ?", prefix));
        params.push(Box::new(after));
    }
    if let Some(before) = pq.created_before {
        conditions.push(format!("{}created <= ?", prefix));
        params.push(Box::new(before));
    }
}

// ---------------------------------------------------------------------------
// Snippet extraction
// ---------------------------------------------------------------------------

pub(crate) fn extract_snippet(content: &str, term: &str) -> String {
    let idx = content.find(term).unwrap_or(0);
    let start = idx.saturating_sub(30);
    let end = (idx + term.len() + 60).min(content.len());
    let slice = &content[start..end];
    let highlighted = slice.replacen(term, &format!(">>>{}<<<", term), 1);
    if start > 0 {
        format!("...{}", highlighted)
    } else {
        highlighted
    }
}

// ---------------------------------------------------------------------------
// Trigram helpers (unused but kept for future use)
// ---------------------------------------------------------------------------

#[allow(dead_code)]
pub(crate) fn get_trigram_candidates(conn: &Connection, pattern: &str, max_results: usize) -> Vec<String> {
    use super::db::generate_trigrams;

    let trigrams = generate_trigrams(pattern);
    if trigrams.is_empty() {
        return vec![];
    }

    let placeholders: String = trigrams.iter().map(|_| "?").collect::<Vec<_>>().join(",");
    let sql = format!(
        "SELECT path FROM trigrams WHERE trigram IN ({}) GROUP BY path HAVING COUNT(DISTINCT trigram) = ? LIMIT ?",
        placeholders
    );

    let mut param_values: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();
    for tri in &trigrams {
        param_values.push(Box::new(tri.clone()));
    }
    param_values.push(Box::new(trigrams.len() as i64));
    param_values.push(Box::new(max_results as i64));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        param_values.iter().map(|p| p.as_ref()).collect();

    let mut paths = Vec::new();
    if let Ok(mut stmt) = conn.prepare(&sql) {
        if let Ok(rows) = stmt.query_map(param_refs.as_slice(), |row| row.get::<_, String>(0)) {
            paths.extend(rows.filter_map(|r| r.ok()));
        }
    }
    paths
}
