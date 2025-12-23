use std::io;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;

pub struct LogReader;

impl LogReader {
    pub async fn tail_file(path: &str, logs: Arc<Mutex<Vec<String>>>) -> tokio::io::Result<()> {
        let file = File::open(path).await?;
        // Check if file errors
        if file.metadata().await.is_err() {
            return Err(io::Error::new(io::ErrorKind::NotFound, "File not found"));
        }

        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            let bytes_read = reader.read_line(&mut line).await?;

            if bytes_read > 0 {
                let mut logs_lock = logs.lock().await;
                logs_lock.push(line.trim_end().to_string());

                // Keep buffer size within 1000 lines
                if logs_lock.len() > 1000 {
                    logs_lock.remove(0);
                }
                line.clear();
            } else {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            }
        }
    }
}
