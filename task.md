# 📋 PIGPEN DEVELOPMENT TASKS

## PHASE 1: INITIALIZATION ✅

- [x] Integrate Mission Prompt (`.ai/prompt.md`)
- [x] Scaffold Next.js 15 App (`src/`)
- [x] Initialize Tauri 2.0 (`src-tauri/`)
- [x] Configure Next.js for SSG (`output: export`)
- [x] Verify Dev Build launch

## PHASE 2: RESIDENCY ✅

- [x] **System Tray Integration**
  - [x] Add default icon
  - [x] Create context menu (Show/Hide, Quit)
- [x] **Global Hotkey**
  - [x] Register `Alt+Space` in Rust
  - [x] Handle window focus/visibility toggling
- [x] **Window Management**
  - [x] Configure `hud` window (frameless, transparent)
  - [x] Configure `dashboard` window (standard, resizable)
  - [x] Implement Mica/Acrylic effects

## PHASE 3: SEARCH ENGINE

- [ ] Implement SQLite FTS5 Scheme
- [ ] Build Weighted Levenshtein Logic
- [ ] Implement Frecency Scoring

## PHASE 4: COMMAND SIGILS

- [ ] Implement prefix parser (`*note`, `*todo`)
- [ ] Build Trie autocomplete

## PHASE 5: THE DASHBOARD

- [ ] Build File Explorer
- [ ] Build Markdown Editor
