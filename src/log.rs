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
        let mut reader = BufReader::new(file).lines();
        while let Ok(Some(line)) = reader.next_line().await {
            let mut logs_lock = logs.lock().await;
            logs_lock.push(line);

            if logs_lock.len() > 100 {
                logs_lock.remove(0);
            }
        }
        Ok(())
    }
}
