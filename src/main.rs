use tokio::sync::CancellationToken;
use tokio::time::Duration;

fn main() {
  let (tx, rx) = mpsc::channel();
  let (token, mut cancellation) = CancellationToken::new();

  // Enable `rt` feature for tokio::spawn
  tokio::spawn(async {
    loop {
      tokio::select! {
        _ = rx.recv() => {
          println!("Received cancellation message");
          break;
        },
        _ = cancellation.cancelled() => {
          println!("Cancellation token triggered");
          break;
        },
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
          println!("Task is still running");
        }
      }
    }
  });

  // Send cancellation message or use cancellation token
  // tx.send(()).unwrap();
  // cancellation.cancel();

  // Wait for task completion (optional)
  // tokio::task::yield_now().await;
}