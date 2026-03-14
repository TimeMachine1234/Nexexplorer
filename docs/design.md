# Nexexplorer Component Design System

A comprehensive guide to every component needed for Nexexplorer. This document tracks all UI components that need custom design and implementation.

## Layout Components

### Core Structure
- **Pane** - Container for a single file browser pane with tabs
- **PaneManager** - Manages dual-pane layout, resizing, and coordination
- **TabBar** - Tab management for each pane (new, close, rename tab)
- **TitleBar** - Window title bar with app name and minimize/maximize/close controls
- **WindowControls** - Window control buttons (minimize, maximize, close)
- **Sidebar** - Left sidebar with drives, quick access, favorites
- **SidebarSection** - Collapsible sections within sidebar (e.g., "Drives", "Favorites")
- **StatusBar** - Bottom status bar showing file count, selection, path info
- **ToolbarActions** - Main toolbar with common actions (copy, paste, delete, etc.)
- **Toolbar** - Container for toolbar buttons and actions
- **MainLayout** - Root layout component wrapping all sections

### Theme & Appearance
- **ThemeProvider** - Theme system wrapper (dark/light mode support)
- **ColorTokens** - CSS design tokens for colors, spacing, typography

## Browser Components

### File Display
- **FileList** - Main file listing (list view)
- **GridView** - Grid/icon view for files
- **FileItem** - Individual file row in list view
- **GridItem** - Individual file card in grid view
- **FileIcon** - File type icon display with custom rendering
- **FilePreviewThumbnail** - Thumbnail generation and display

### Navigation
- **BreadcrumbBar** - Navigation breadcrumbs showing current path
- **NavigationButtons** - Back/forward/up navigation buttons
- **AddressBar** - Path input bar (editable location bar)

### Filtering & Search
- **FilterBar** - Filter controls (sort, view mode, group options)
- **FolderFilterBar** - In-memory folder filter/search bar
- **SearchOverlay** - Global search UI overlay (Ctrl+F)
- **SearchResults** - Results display for global search
- **SearchResultItem** - Individual search result row

### File Operations
- **TransferQueue** - Copy/move/delete progress queue
- **ConflictDialog** - File conflict resolution UI (overwrite/rename/skip)
- **ProgressBar** - File operation progress indicator
- **TransferItem** - Individual transfer item in queue

## Preview Components

### Preview Panel
- **PreviewPanel** - Side preview panel container
- **PreviewToolbar** - Toolbar for preview actions
- **PreviewBody** - Content area for previews
- **QuickPreview** - Quick preview for common file types

### Preview Types
- **ImagePreview** - Image viewer with zoom, rotate, fit options
- **ImageViewport** - Canvas/viewport for image display with controls
- **VideoPreview** - Video player with standard controls
- **AudioPlayer** - Audio player with waveform visualization
- **PdfPreview** - PDF viewer with page navigation
- **TextPreview** - Text/code preview with syntax highlighting
- **CodePreview** - Code file preview with line numbers
- **ArchivePreview** - Archive contents listing
- **BinaryPreview** - Hex viewer for binary files

## Dialog & Modal Components

### Dialogs
- **Dialog** - Base dialog component
- **AlertDialog** - Alert/confirmation dialog
- **ConfirmDialog** - Confirmation dialog with Yes/No
- **InputDialog** - Input prompt dialog
- **FilePropertiesDialog** - File properties/metadata viewer
- **SettingsDialog** - App settings modal
- **KeyboardShortcutsDialog** - Keyboard shortcuts reference
- **AboutDialog** - About app dialog

## Common UI Components

### Input Controls
- **Button** - Standard button with variants (primary, secondary, danger, ghost)
- **IconButton** - Button with icon only
- **TextInput** - Text input field
- **SearchInput** - Search field variant
- **NumberInput** - Number input field
- **Checkbox** - Checkbox control
- **RadioButton** - Radio button group
- **Toggle** - Toggle switch control
- **Dropdown** - Dropdown/select menu
- **ComboBox** - Searchable dropdown
- **FileInput** - File picker input
- **ColorPicker** - Color selection input

### Display Components
- **Badge** - Label badge for tags, status
- **Label** - Text label
- **Text** - Text display with variants
- **Heading** - Heading component (h1-h6)
- **Icon** - Icon display component
- **Avatar** - User/file avatar display
- **Spinner** - Loading spinner
- **Skeleton** - Skeleton loader
- **Divider** - Horizontal divider line
- **Separator** - Vertical separator

### Feedback
- **Toast** - Toast notification
- **Snackbar** - Bottom snackbar message
- **ProgressBar** - Progress indicator
- **Tooltip** - Hover tooltip
- **Popover** - Popover content
- **Dropdown** - Dropdown menu

### Cards & Containers
- **Card** - Card container
- **Panel** - Panel container
- **ScrollArea** - Scrollable container with custom scrollbar

## Context Menu Components

### Menus
- **ContextMenu** - Right-click context menu
- **ContextMenuItem** - Menu item in context menu
- **ContextMenuDivider** - Divider in context menu
- **ContextMenuSubMenu** - Nested submenu
- **Menu** - Standard menu component
- **MenuItem** - Menu item
- **MenuDivider** - Menu divider

## Home Screen Components

### Dashboard
- **HomeView** - Home screen/dashboard view
- **QuickAccessGrid** - Quick access folders grid
- **QuickAccessItem** - Individual quick access item
- **RecentFiles** - Recent files section
- **RecentFileItem** - Individual recent file
- **DriveCard** - Drive display card
- **StorageIndicator** - Drive storage usage indicator

## Utility Components

### Helpers
- **EmptyState** - Empty state display (no files, no results)
- **ErrorBoundary** - Error boundary for error handling
- **Loading** - Loading state component
- **Portal** - Portal for modals and overlays
- **DragDropZone** - Drag and drop zone
- **Resizable** - Resizable container
- **VirtualScroller** - Virtual scrolling for large lists

## Command Palette Components

### Command Execution
- **CommandPalette** - Command palette UI (Ctrl+K)
- **CommandInput** - Command search input
- **CommandResult** - Command result item
- **CommandCategory** - Grouped command category

## Status Components

### Status Indicators
- **ConnectionStatus** - Network/connection status
- **SyncStatus** - Sync status indicator
- **IndexingStatus** - Search index status
- **ActivityIndicator** - Current activity indicator

## Typography & Spacing

### Text Styles
- **Typography** - Text style definitions
- **Heading1** - H1 style
- **Heading2** - H2 style
- **Heading3** - H3 style
- **Body** - Body text style
- **Caption** - Caption/small text style
- **Code** - Code/monospace style

### Spacing System
- **Spacing tokens** - Consistent spacing scale (4px, 8px, 12px, 16px, 24px, 32px, etc.)

## Icon System

### Icon Components
- **FileTypeIcon** - Icon by file extension
- **FolderIcon** - Folder icon variants
- **SystemIcon** - System icon (documents, downloads, etc.)
- **CustomIcon** - Custom SVG icon wrapper
- **IconSet** - Collection of all icons used in app

## Animation Components

### Transitions & Animations
- **FadeIn** - Fade in animation
- **SlideIn** - Slide in animation
- **ScaleIn** - Scale in animation
- **Transition** - Generic transition wrapper
- **AnimatedNumber** - Animated number counter

## Component Status Tracking

| Component | Status | Design | Implementation | Notes |
|-----------|--------|--------|-----------------|-------|
| | Planned | ⬜ | ⬜ | |

## Design Principles

1. **Modern Aesthetic** - Clean, minimal design with careful attention to spacing and typography
2. **Consistency** - All components follow design tokens and system
3. **Accessibility** - Keyboard navigation, screen reader support, sufficient contrast
4. **Performance** - Components are lightweight, efficient rendering
5. **Minimal Chrome** - UI stays out of the way, maximizes content space
6. **Dark Mode Support** - All components work in light and dark themes

## Icon Rule

**All icons are inline SVG. No emoji. No external icon libraries.**

Emoji render inconsistently across Windows versions and fonts. All icons use `stroke="currentColor"` with consistent `stroke-width="1.4"` on a `16×16 viewBox`. SVG icon paths live inline in each component that uses them (or in a `Record<string, string>` dictionary for sets of related icons).

## Squircle Token Rule

**Never use `--radius-*` CSS variables. Always use `--sq-*` squircle tokens.**

| Token | Value | Use case |
|---|---|---|
| `--sq-xs` | 4px | Tiny elements, badges |
| `--sq-sm` | 6px | Chips, crumb buttons |
| `--sq-md` | 10px | Buttons, inputs, small panels |
| `--sq-lg` | 14px | Cards, menus |
| `--sq-xl` | 18px | Address bar, larger panels |
| `--sq-2xl` | 22px | Modals, dialogs |
| `--sq-icon` | 28% | Square icons/avatars |
| `--sq-full` | 9999px | Pills, toggles |

## Animation / Timing Rule

Use `requestAnimationFrame`/`cancelAnimationFrame` instead of `setTimeout` for all DOM-synchronization work (focus, scroll state, rect measurement). This aligns work with the browser's paint cycle and avoids the 0–4ms jitter of `setTimeout(fn, 0)`.

## Next Steps

1. Create design mockups for each component category
2. Establish color palette and typography system
3. Build component library with Storybook/preview
4. Implement components progressively
5. Test accessibility and performance
