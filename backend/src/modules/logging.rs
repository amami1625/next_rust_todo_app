use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

/// ロギングを初期化
/// 既に初期化されている場合は何もしない
pub fn init() {
    let _ = tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "todo_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_does_not_panic() {
        // 初回の初期化
        init();

        // 2回目の初期化でもパニックしない
        init();
    }

    #[test]
    fn test_init_can_be_called_multiple_times() {
        // 複数回呼んでも安全
        for _ in 0..3 {
            init();
        }
    }
}
