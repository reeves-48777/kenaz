//! Concurrency utilities for safe thread management.

/// RAII guard to ensure the concurrency permit is always returned,
/// even if the thread panics or returns an error early.
///
/// When the `PermitGuard` goes out of scope, it automatically sends a unit `()`
/// back into the `mpsc::sync_channel`, releasing the slot for the next thread.
pub struct PermitGuard {
    pub tx: std::sync::Arc<std::sync::mpsc::SyncSender<()>>,
}
impl Drop for PermitGuard {
    fn drop(&mut self) {
        let _ = self.tx.send(());
    }
}
