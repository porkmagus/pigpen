# 🗺️ PIGPEN: THE MASTER ORCHESTRATION ROADMAP (WIN-X1)
**Project Identity:** Pigpen (Raycast-Obsidian Desktop Mashup)
**Target Platform:** Windows 10/11 (WebView2 / Rust / Tauri)
**Lead Methodology:** Local-First High-Performance Engineering

---

## 🚩 MILESTONE 1: THE IGNITION & SYSTEM RESIDENCY
**Primary Phase:** Infrastructure & Environment Scaffolding
**Active Role Activation:** Master Software Architect & Master Copycat Reverse Engineer

### 1.1 Mission Directive
The objective of this milestone is to establish the "Resident Pulse" of Pigpen. You are to initialize the Tauri/Rust backend and configure the application to live permanently in the Windows System Tray. This is the foundation upon which all fuzzy search and knowledge management logic will rest.

### 1.2 Architectural Execution
- **Role [Master Software Architect]:** Execute the forensic analysis of the `REFERENCE.md` and `MASTER_ARCHITECT_REF.md`. Initialize the project structure using the Tauri CLI. Configure `tauri.conf.json` to define the "Dual-Surface" windowing strategy: the `hud` (frameless/ephemeral) and the `dashboard` (persistent/standard).
- **Role [Master Copycat Reverse Engineer]:** Implement the Windows-native aesthetics. You must apply "Mica" or "Acrylic" vibrancy effects to the HUD surface using the `window-vibrancy` crate. Ensure the window shadow and corner radius (12px for Win11) match the premium "Raycast" aesthetic perfectly.
- **System Integration:** Register the Global Shortcut listener for `Alt + Space`. This must be handled via a dedicated Rust thread to ensure the UI thread is never blocked.

### 1.3 Documentation & Compliance
- **Manifest:** Log the initial Architectural Decision Record (ADR-001) regarding the multi-window process model.
- **Health:** Update `HEALTH.md` to reflect the initial RAM footprint and startup latency.



---

## 🚩 MILESTONE 2: THE ENGINE & LOGIC DECRYPTION
**Primary Phase:** SQLite FTS5 Implementation & Algorithmic Engineering
**Active Role Activation:** Master Software Architect & Master Refactorer and Migrator

### 2.1 Mission Directive
The objective is to build the "Brain" of Pigpen. You must bridge the gap between the local Markdown vault and the high-speed HUD search results. This phase is about data integrity, retrieval speed, and mathematical ranking.

### 2.2 Algorithmic Execution
- **Role [Master Software Architect]:** Design the SQLite FTS5 schema in Rust. Implement the `trigram` tokenizer to ensure that even partial substring matches in your notes return results. You must architect the "File System Watcher" logic using the `notify` crate, ensuring that any external edit to the vault triggers an atomic incremental update to the SQLite index.
- **Role [Master Refactorer and Migrator]:** Implement the Weighted Levenshtein and Frecency logic specified in `REFERENCE.md`. You must ensure the search function is $O(\log n)$ and that data serialization between the Rust backend and Next.js frontend is handled via zero-copy buffers or optimized JSON-RPC to avoid parsing bottlenecks.
- **Data Safety:** Implement the "Atomic Write" pattern for the `*note` sigil to prevent file corruption in the event of a Windows system crash.

### 2.3 Documentation & Compliance
- **Manifest:** Log ADR-002 regarding the choice of SQLite FTS5 and the trigram configuration.
- **Health:** Update the "Search Latency" benchmark. The goal is <10ms for 5,000 notes.



---

## 🚩 MILESTONE 3: THE INTERFACE & COMMAND ROUTING
**Primary Phase:** HUD UI Implementation & Sigil Parsing
**Active Role Activation:** Master Copycat Reverse Engineer & Master Software Architect

### 3.1 Mission Directive
The objective is to turn the "Brain" into a usable product. You will build the high-density HUD interface and the command-routing logic that allows the user to capture information without leaving their current context.

### 3.2 UI/UX Execution
- **Role [Master Copycat Reverse Engineer]:** Build the Next.js search interface. Replicate the high-density list view of Raycast, including subtle typography (Inter/Segoe UI Variable) and snappy selection states. Every keystroke must be reflected in the UI in under 8ms.
- **Role [Master Software Architect]:** Implement the "Sigil Command Parser" in the Frontend. Write the routing logic for:
  - `*note`: Immediate file creation.
  - `*todo`: Append logic for `TASKS.md`.
  - `*open`: Shell-execution for Windows apps/URLs.
- **Navigation:** Implement full keyboard-driven navigation (`Arrow Keys`, `Enter`, `Esc`). The HUD must feel like an extension of the Windows OS.

### 3.3 Documentation & Compliance
- **Manifest:** Document the UI component library choices and the state management strategy for the bi-directional HUD/Dashboard sync.
- **Health:** Update the "Input Lag" and "UI Responsiveness" metrics.

---

## 🚩 MILESTONE 4: THE GAUNTLET & PRODUCTION HARDENING
**Primary Phase:** QA Stress Testing & Binary Optimization
**Active Role Activation:** Master QA Tester & Master Refactorer and Migrator

### 4.1 Mission Directive
The objective is to harden Pigpen for production. You must ensure the application is "Bulletproof" on Windows 10/11 and that the binary size is optimized for distribution.

### 4.2 Hardening Execution
- **Role [Master QA Tester]:** Execute "The Pigpen Gauntlet." You must write and run PowerShell-based stress tests to simulate 50+ concurrent file edits and high-frequency `Alt + Space` triggers. Verify focus-stealing behavior while full-screen applications are active.
- **Role [Master Refactorer and Migrator]:** Conduct a "Zero-Placeholder" audit. Remove all dead code, optimize Rust imports, and enable Link-Time Optimization (LTO). Strip all debug symbols to ensure the final Windows installer (`.msi` or `.exe`) is under 12MB.
- **Stability:** Conduct a memory leak audit. Ensure the background resident process does not climb above the 60MB RAM budget during 24 hours of uptime.

### 4.3 Documentation & Compliance
- **Manifest:** Finalize the project log. Note all remaining technical debt and the final build version.
- **Health:** Finalize the dashboard. Every metric must be green (🟢) before the project is considered "Complete."



---

## 🏁 UNIVERSAL EXECUTION MANDATE
You are to proceed through these milestones sequentially. You are strictly forbidden from moving to a new milestone until the current one is 100% verified and logged in both `MANIFEST.md` and `HEALTH.md`. Every role—**Architect, Refactorer, QA, and Copycat**—must be invoked at the appropriate junction to ensure Pigpen meets the highest engineering standards of 2026.

**STATUS:** STANDBY  
**REPLY WITH "MISSION INITIATED" TO BEGIN MILESTONE 1. GO.**