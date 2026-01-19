# 📕 PIGPEN: THE MASTER TECHNICAL SPECIFICATION (WIN-X1)
**Project Name:** Pigpen
**Target Environment:** Windows 10/11 (WebView2 / Win32 API)

## 1. PRODUCT DNA: THE RAYCAST-OBSIDIAN MASHUP
Pigpen is a hybrid "Knowledge Launcher." It combines the heads-up utility of a global command bar with the persistent deep-work environment of a markdown-based knowledge vault.

### 1.1 The HUD UI (Surface A)
The **Master Copycat Reverse Engineer** must replicate the "high-density" feel of Raycast:
- **Design Tokens:** - **Windows 11 Mica / Windows 10 Acrylic:** Background vibrancy is mandatory.
  - **Corner Radius:** 8px for Windows 10; 12px for Windows 11.
  - **Shadows:** Soft, deep shadows (0 20px 50px rgba(0,0,0,0.3)).
- **Focus Logic:** The search input must be focused programmatically the microsecond the window appears.

### 1.2 The Dashboard (Surface B)
The **Master Software Architect** defines this as the persistent organizational layer:
- **Navigation:** Folder-based explorer on the left; Markdown preview/editor in the center.
- **Linking:** Support for `[[Wiki-links]]` for bi-directional knowledge management.

## 2. THE SEARCH ENGINE: FUZZY LOGIC & RANKING
This is the core engine of Pigpen, to be built by the **Master Implementor** and optimized by the **Master Refactorer and Migrator**.

### 2.1 Weighted Levenshtein Logic
The search engine must not rely on exact matches. It must use a weighted distance algorithm:
- **Prefix Match:** 1.0 multiplier.
- **Title Match:** 0.8 multiplier.
- **Content Match:** 0.4 multiplier.
- **Distance Penalty:** Subtract 0.1 for every Levenshtein distance point beyond 0.

### 2.2 The Frecency Formula
Sort results by $S = (M \cdot 0.7) + (F \cdot 0.2) + (R \cdot 0.1)$.
- **M (Match Quality):** The weighted Levenshtein score.
- **F (Frequency):** Count of times the item has been selected in Pigpen.
- **R (Recency):** Time elapsed since last selection (logarithmic decay).

## 3. DATA ARCHITECTURE: THE SQLITE FTS5 VAULT
Pigpen treats the user's Markdown vault as the source of truth, but uses SQLite for high-speed indexing.
- **Schema:**
  - `notes`: `id`, `path`, `title`, `content_preview`, `tags`.
  - `frecency`: `item_id`, `usage_count`, `last_accessed`.
- **Tokenizer:** Use the `trigram` tokenizer for FTS5 to enable partial-word and substring matching.

## 4. THE COMMAND SIGILS (HUD CLI)
The HUD must parse the following sigils for rapid data entry:
- `*note [text]`: Creates a file in `/vault/inbox/` using the first 5 words as the filename.
- `*todo [text]`: Appends `- [ ] [text]` to a global `TASKS.md` file.
- `*open [query]`: Searches for local Windows `.exe` files or URLs.

## 5. ROLE-SPECIFIC TECHNICAL REQUIREMENTS
- **Master QA Tester:** Must verify that the `Alt + Space` hotkey works while full-screen games or videos are running.
- **Master Refactorer and Migrator:** Must optimize the Rust binary using LTO (Link-Time Optimization) and symbol stripping to keep the `.exe` size under 12MB.
- **Master Copycat:** Must ensure that the HUD font (Inter or Segoe UI Variable) matches the system-level anti-aliasing perfectly.

## 6. DOCUMENTATION STANDARDS
- **MANIFEST.md:** Every model handover must be documented.
- **HEALTH.md:** Performance metrics (latency in ms) must be updated after every merge.