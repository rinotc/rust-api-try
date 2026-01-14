
#[derive(Debug, thiserror::Error)]
pub enum AppError<E> {
    /// ビジネスロジック上のエラー（NotFound, Duplicateなど）。
    /// メソッドごとに異なる
    #[error(transparent)]
    Business(E),

    /// システム的なエラー（DB接続エラー、ネットワークエラーなど）
    #[error("System error occurred.")]
    System(#[from] anyhow::Error)
}