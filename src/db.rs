use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use std::path::Path;

use crate::LogEntry;

pub struct LogDb {
    conn: Connection,
}

impl LogDb {
    pub fn open(path: &Path) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init()?;
        Ok(db)
    }

    fn init(&self) -> Result<()> {
        self.conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                tool TEXT NOT NULL,
                session_id TEXT NOT NULL,
                action TEXT NOT NULL,
                content TEXT NOT NULL,
                metadata TEXT
            );
            CREATE INDEX IF NOT EXISTS idx_entries_tool ON entries(tool);
            CREATE INDEX IF NOT EXISTS idx_entries_session ON entries(session_id);
            CREATE VIRTUAL TABLE IF NOT EXISTS entries_fts USING fts5(content, session_id UNINDEXED);

            CREATE TRIGGER IF NOT EXISTS entries_ai AFTER INSERT ON entries BEGIN
                INSERT INTO entries_fts(rowid, content, session_id)
                VALUES (new.id, new.content, new.session_id);
            END;

            CREATE TRIGGER IF NOT EXISTS entries_ad AFTER DELETE ON entries BEGIN
                INSERT INTO entries_fts(entries_fts, rowid, content, session_id)
                VALUES ('delete', old.id, old.content, old.session_id);
            END;
            "
        )?;
        Ok(())
    }

    pub fn insert_entry(&self, entry: &LogEntry) -> Result<()> {
        self.conn.execute(
            "INSERT INTO entries (timestamp, tool, session_id, action, content, metadata)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                entry.timestamp.to_rfc3339(),
                &entry.tool,
                &entry.session_id,
                &entry.action,
                &entry.content,
                entry.metadata.as_ref().map(|m| m.to_string()),
            ],
        )?;
        Ok(())
    }

    pub fn list_entries(&self, tool: Option<&str>, limit: usize) -> Result<Vec<LogEntry>> {
        let sql = if let Some(_t) = tool {
            "SELECT timestamp, tool, session_id, action, content, metadata
             FROM entries WHERE tool = ?1
             ORDER BY timestamp DESC LIMIT ?2"
        } else {
            "SELECT timestamp, tool, session_id, action, content, metadata
             FROM entries
             ORDER BY timestamp DESC LIMIT ?1"
        };

        let mut stmt = self.conn.prepare(sql)?;
        let rows = if let Some(t) = tool {
            stmt.query_map(params![t, limit], Self::map_row)?
        } else {
            stmt.query_map(params![limit], Self::map_row)?
        };

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn search(&self, query: &str) -> Result<Vec<LogEntry>> {
        let mut stmt = self.conn.prepare(
            "SELECT e.timestamp, e.tool, e.session_id, e.action, e.content, e.metadata
             FROM entries_fts fts
             JOIN entries e ON e.id = fts.rowid
             WHERE entries_fts MATCH ?1
             ORDER BY rank
             LIMIT 50"
        )?;
        let rows = stmt.query_map([query], Self::map_row)?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    pub fn list_sessions(&self) -> Result<Vec<(String, String, i64, String)>> {
        let mut stmt = self.conn.prepare(
            "SELECT session_id, tool, COUNT(*) as cnt, MAX(timestamp) as latest
             FROM entries
             GROUP BY session_id, tool
             ORDER BY latest DESC"
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
        })?;
        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    fn map_row(row: &rusqlite::Row) -> std::result::Result<LogEntry, rusqlite::Error> {
        let ts_str: String = row.get(0)?;
        let ts = DateTime::parse_from_rfc3339(&ts_str)
            .map(|d| d.with_timezone(&Utc))
            .unwrap_or_else(|_| Utc::now());

        Ok(LogEntry {
            timestamp: ts,
            tool: row.get(1)?,
            session_id: row.get(2)?,
            action: row.get(3)?,
            content: row.get(4)?,
            metadata: row.get::<_, Option<String>>(5)?.and_then(|s| serde_json::from_str(&s).ok()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn test_db() -> LogDb {
        let tmp = PathBuf::from("/tmp/agent-console-test.db");
        let _ = std::fs::remove_file(&tmp);
        LogDb::open(&tmp).unwrap()
    }

    #[test]
    fn test_insert_and_search() {
        let db = test_db();
        let entry = LogEntry {
            timestamp: Utc::now(),
            tool: "test".into(),
            session_id: "session-1".into(),
            action: "write".into(),
            content: "Hello world from agent console".into(),
            metadata: None,
        };
        db.insert_entry(&entry).unwrap();

        let results = db.search("agent").unwrap();
        assert_eq!(results.len(), 1);
        assert!(results[0].content.contains("agent"));
    }
}
