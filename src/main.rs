use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

mod db;

use db::LogDb;

#[derive(Parser)]
#[command(name = "agent-console")]
#[command(about = "Unified log timeline for your agent sessions.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Import logs from a tool
    Import {
        /// Tool name (cline, aider, claude-code, goose, generic)
        #[arg(short, long)]
        tool: String,
        /// Path to log file or directory
        path: PathBuf,
    },
    /// Show timeline of agent sessions
    Timeline {
        /// Filter by tool
        #[arg(short, long)]
        tool: Option<String>,
        /// Limit results
        #[arg(short, long, default_value = "50")]
        limit: usize,
    },
    /// Search across all logs
    Search {
        /// Search query
        query: String,
    },
    /// List imported sessions
    Sessions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub tool: String,
    pub session_id: String,
    pub action: String,
    pub content: String,
    pub metadata: Option<serde_json::Value>,
}

fn parse_generic_log(path: &PathBuf) -> Result<Vec<LogEntry>> {
    let content = fs::read_to_string(path)?;
    let mut entries = Vec::new();
    for line in content.lines() {
        if let Ok(entry) = serde_json::from_str::<LogEntry>(line) {
            entries.push(entry);
        }
    }
    Ok(entries)
}

fn parse_aider_log(path: &PathBuf) -> Result<Vec<LogEntry>> {
    let content = fs::read_to_string(path)?;
    let mut entries = Vec::new();
    let session_id = format!("aider-{}", chrono::Utc::now().timestamp());

    for (i, line) in content.lines().enumerate() {
        entries.push(LogEntry {
            timestamp: chrono::Utc::now(),
            tool: "aider".into(),
            session_id: session_id.clone(),
            action: "output".into(),
            content: line.into(),
            metadata: Some(serde_json::json!({ "line": i })),
        });
    }
    Ok(entries)
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let db_path = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("agent-console")
        .join("logs.db");
    std::fs::create_dir_all(db_path.parent().unwrap())?;
    let db = LogDb::open(&db_path)?;

    match cli.command {
        Commands::Import { tool, path } => {
            let entries = match tool.as_str() {
                "aider" => parse_aider_log(&path)?,
                "generic" => parse_generic_log(&path)?,
                _ => {
                    // Try generic first, then fallback
                    parse_generic_log(&path).or_else(|_| parse_aider_log(&path))?
                }
            };

            let count = entries.len();
            for entry in entries {
                db.insert_entry(&entry)?;
            }
            println!("Imported {} entries from {}.", count, tool);
        }

        Commands::Timeline { tool, limit } => {
            let entries = db.list_entries(tool.as_deref(), limit)?;
            if entries.is_empty() {
                println!("No entries found.");
                return Ok(());
            }

            let mut current_session = String::new();
            for e in entries {
                if e.session_id != current_session {
                    current_session = e.session_id.clone();
                    println!("\n=== Session: {} ===", current_session);
                }
                let ts = e.timestamp.format("%H:%M:%S").to_string();
                println!("[{}] [{}] {} | {}", ts, e.tool, e.action, e.content.chars().take(80).collect::<String>());
            }
        }

        Commands::Search { query } => {
            let entries = db.search(&query)?;
            if entries.is_empty() {
                println!("No results for '{}'.", query);
            } else {
                println!("Found {} result(s):\n", entries.len());
                for e in entries {
                    println!(
                        "[{}] {} | {}: {}",
                        e.timestamp.format("%Y-%m-%d %H:%M:%S"),
                        e.tool,
                        e.action,
                        e.content.chars().take(120).collect::<String>()
                    );
                }
            }
        }

        Commands::Sessions => {
            let sessions = db.list_sessions()?;
            if sessions.is_empty() {
                println!("No sessions found.");
            } else {
                for (id, tool, count, latest) in sessions {
                    println!("{} | {} | {} entries | last: {}", id, tool, count, latest);
                }
            }
        }
    }

    Ok(())
}
