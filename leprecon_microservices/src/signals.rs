use tokio::signal;

/// Handles shutdown signals
///
/// ### Current signals:
/// - Ctrl+c
/// - SIGTERM
pub async fn shutdown_signal() {
    // Ctrl+c
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    // SIGTERM
    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            println!("\nShutting down...");
        },
        _ = terminate => {
            println!("Shutting down...");
        },
    }
}
