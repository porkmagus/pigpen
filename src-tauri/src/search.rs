use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub id: String,
    pub path: String,
    pub title: String,
    pub preview: String,
    pub score: f64,
}

pub struct SearchEngine {
    pub conn: Connection,
}

impl SearchEngine {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)?;

        // Initialize Schema
        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             CREATE VIRTUAL TABLE IF NOT EXISTS notes USING fts5(
                id, 
                path, 
                title, 
                content, 
                tags,
                tokenize='trigram'
             );
             CREATE TABLE IF NOT EXISTS frecency (
                item_id TEXT PRIMARY KEY,
                usage_count INTEGER DEFAULT 0,
                last_accessed INTEGER
             );",
        )?;

        Ok(SearchEngine { conn })
    }

    pub fn index_vault<P: AsRef<Path>>(&self, vault_path: P) -> Result<usize> {
        let mut count = 0;
        let mut stmt = self.conn.prepare(
            "INSERT OR REPLACE INTO notes (id, path, title, content, tags) VALUES (?1, ?2, ?3, ?4, ?5)"
        )?;

        for entry in WalkDir::new(vault_path).into_iter().filter_map(|e| e.ok()) {
            if entry.path().extension().and_then(|s| s.to_str()) == Some("md") {
                let path = entry.path().to_string_lossy().to_string();
                let title = entry.file_name().to_string_lossy().replace(".md", "");
                let content = std::fs::read_to_string(entry.path()).unwrap_or_default();

                // Simple ID strategy for now: path hash or path itself
                let id = path.clone();

                stmt.execute(params![id, path, title, content, ""])?;
                count += 1;
            }
        }
        Ok(count)
    }

    pub fn search(&self, query: &str) -> Result<Vec<SearchResult>> {
        // Basic FTS5 search with frecency placeholder
        let mut stmt = self.conn.prepare(
            "SELECT id, path, title, snippet(notes, 3, '...', '...', '...', 10) as preview, rank 
             FROM notes 
             WHERE notes MATCH ?1 
             ORDER BY rank LIMIT 25",
        )?;

        let rows = stmt.query_map(params![query], |row| {
            Ok(SearchResult {
                id: row.get(0)?,
                path: row.get(1)?,
                title: row.get(2)?,
                preview: row.get(3)?,
                score: row.get(4)?,
            })
        })?;

        let mut results = Vec::new();
        for row in rows {
            results.push(row?);
        }
        Ok(results)
    }
}
