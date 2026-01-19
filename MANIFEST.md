# 📑 PROJECT MANIFEST: PIGPEN DEVELOPMENT LOG

**Project Identity:** Pigpen (Windows 10/11 Local-First Launcher)

## 1. ARCHITECTURAL DECISION RECORDS (ADR)

This section logs the "Why" behind the Pigpen system architecture.

### ADR-001: Multi-Window Resident Strategy

- **Decision:** Use a single Tauri process to manage two distinct window labels (`hud` and `dashboard`).
- **Justification:** Shared memory between the Tray and the UI surfaces is critical for <10ms response times.
- **Architect:** Master Software Architect.

### ADR-002: SQLite FTS5 for Vault Indexing

- **Decision:** Implement a trigram-based SQLite index instead of raw Markdown parsing on every search.
- **Justification:** Scalability. As the Pigpen vault grows to 10k+ notes, $O(n)$ search becomes unusable.
- **Architect:** Master Software Architect.

### ADR-003: Initialization Tech Stack

- **Decision:** Next.js 15 (App Router, SSG) + Tauri 2.0.
- **Justification:** Next.js provides robust routing for the Dashboard. SSG (`output: export`) enables local-first deployment. Tauri 2.0 is the project mandate.
- **Architect:** Master Software Architect.

## 2. PERSONALITY & ROLE ACTIVATION LOG

| Date       | Task                   | Active Role      | Model | Status      |
| ---------- | ---------------------- | ---------------- | ----- | ----------- |
| 2026-01-19 | Phase 1 Initialization | Master Architect | AI    | ✅ Complete |
| 2026-01-19 | Phase 2 Residency      | Master Architect | AI    | ✅ Complete |

## 3. TECHNICAL DEBT & REFACTORING BACKLOG

This section is managed by the **Master Refactorer and Migrator**.

## 4. CHANGE LOG & VERSIONING

- **v0.1.0:** Project Initialized. Next.js 15 + Tauri 2.0 scaffolding complete.
- **Goal:** Phase 2 - System Tray & Hotkeys.
