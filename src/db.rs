use std::fs;
use std::path::PathBuf;

use color_eyre::eyre::{Result, WrapErr};
use rusqlite::Connection;
use rusqlite_migration::{M, Migrations};

use crate::model::{IssueSummary, Priority, Status};

const SCHEMA_V1: &str = r#"
CREATE TABLE IF NOT EXISTS issues (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    prefix      TEXT NOT NULL DEFAULT 'ST',
    number      INTEGER NOT NULL DEFAULT 0,
    title       TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    status      TEXT NOT NULL DEFAULT 'todo'
                CHECK(status IN ('todo', 'in_progress', 'done')),
    priority    INTEGER NOT NULL DEFAULT 0
                CHECK(priority BETWEEN 0 AND 3),
    sort_order  REAL NOT NULL DEFAULT 0.0,
    created_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now', 'localtime')),
    updated_at  TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%S', 'now', 'localtime'))
);

CREATE TABLE IF NOT EXISTS labels (
    id    INTEGER PRIMARY KEY AUTOINCREMENT,
    name  TEXT NOT NULL UNIQUE,
    color TEXT NOT NULL DEFAULT '#888888'
);

CREATE TABLE IF NOT EXISTS issue_labels (
    issue_id INTEGER NOT NULL REFERENCES issues(id) ON DELETE CASCADE,
    label_id INTEGER NOT NULL REFERENCES labels(id) ON DELETE CASCADE,
    PRIMARY KEY (issue_id, label_id)
);

CREATE INDEX idx_issues_status_sort ON issues(status, sort_order);

CREATE TRIGGER set_issue_number
AFTER INSERT ON issues
FOR EACH ROW
WHEN NEW.number = 0
BEGIN
    UPDATE issues SET number = (
        SELECT COALESCE(MAX(number), 0) + 1 FROM issues WHERE prefix = NEW.prefix
    ) WHERE id = NEW.id;
END;

CREATE TRIGGER update_issues_timestamp
AFTER UPDATE ON issues
FOR EACH ROW
BEGIN
    UPDATE issues SET updated_at = strftime('%Y-%m-%dT%H:%M:%S', 'now', 'localtime')
    WHERE id = NEW.id;
END;
"#;

fn migrations() -> Migrations<'static> {
    Migrations::new(vec![M::up(SCHEMA_V1)])
}

fn db_path() -> PathBuf {
    let dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("slate");
    fs::create_dir_all(&dir).ok();
    dir.join("kanban.db")
}

pub struct Db {
    conn: Connection,
}

impl Db {
    pub fn open() -> Result<Self> {
        let path = db_path();
        let mut conn = Connection::open(&path)
            .wrap_err_with(|| format!("Failed to open database at {}", path.display()))?;

        conn.execute_batch(
            "PRAGMA journal_mode = WAL;
             PRAGMA foreign_keys = ON;
             PRAGMA busy_timeout = 5000;",
        )
        .wrap_err("Failed to set PRAGMAs")?;

        migrations()
            .to_latest(&mut conn)
            .wrap_err("Failed to run migrations")?;

        Ok(Db { conn })
    }

    pub fn load_column(&self, status: Status) -> Result<Vec<IssueSummary>> {
        let mut stmt = self.conn.prepare_cached(
            "SELECT id, prefix, number, title, status, priority, sort_order, created_at
             FROM issues
             WHERE status = ?1
             ORDER BY sort_order ASC, id ASC",
        )?;

        let rows = stmt.query_map([status.as_str()], |row| {
            let prefix: String = row.get(1)?;
            let number: i64 = row.get(2)?;
            let status_str: String = row.get(4)?;
            let priority_val: i32 = row.get(5)?;
            Ok(IssueSummary {
                id: row.get(0)?,
                issue_id: format!("{prefix}-{number}"),
                title: row.get(3)?,
                status: Status::try_from(status_str.as_str()).unwrap_or(Status::Todo),
                priority: Priority::from_i32(priority_val),
                sort_order: row.get(6)?,
                created_at: row.get(7)?,
            })
        })?;

        let mut issues = Vec::new();
        for row in rows {
            issues.push(row?);
        }
        Ok(issues)
    }

    pub fn count_by_status(&self, status: Status) -> Result<usize> {
        let count: i64 = self.conn.query_row(
            "SELECT COUNT(*) FROM issues WHERE status = ?1",
            [status.as_str()],
            |row| row.get(0),
        )?;
        Ok(count as usize)
    }

    pub fn create_issue(&self, title: &str, status: Status) -> Result<i64> {
        let sort_order = self.next_sort_order(status)?;
        self.conn.execute(
            "INSERT INTO issues (title, status, number, sort_order) VALUES (?1, ?2, 0, ?3)",
            rusqlite::params![title, status.as_str(), sort_order],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn delete_issue(&self, id: i64) -> Result<()> {
        self.conn
            .execute("DELETE FROM issues WHERE id = ?1", [id])?;
        Ok(())
    }

    pub fn move_issue(&self, id: i64, new_status: Status) -> Result<()> {
        let sort_order = self.next_sort_order(new_status)?;
        self.conn.execute(
            "UPDATE issues SET status = ?1, sort_order = ?2 WHERE id = ?3",
            rusqlite::params![new_status.as_str(), sort_order, id],
        )?;
        Ok(())
    }

    fn next_sort_order(&self, status: Status) -> Result<f64> {
        let max: f64 = self
            .conn
            .query_row(
                "SELECT COALESCE(MAX(sort_order), 0.0) FROM issues WHERE status = ?1",
                [status.as_str()],
                |row| row.get(0),
            )
            .unwrap_or(0.0);
        Ok(max + 1.0)
    }
}
