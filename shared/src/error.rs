use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;
// thiserrorはカスタムエラーを作成するのに有用。ボイラープレート削減に役立つ。

#[derive(Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    //`{0}`にバリアント（Enumの各選択肢のこと）が持つ最初のフィールドの値を文字列として埋め込む
    UnprocessableEntity(String),
    #[error("{0}")]
    EntityNotFound(String),
    #[error("{0}")]
    ValidationError(#[from] garde::Report),//#[from]ではsqlx::Errorなどの別のエラー型をAppErrorに変換する処理を自動生成。
    #[error("トランザクションを実行できませんでした。")]
    TransactionError(#[source] sqlx::Error),
    #[error("データベース処理実行中にエラーが発生しました。")]
    SpecificOperationError(#[source] sqlx::Error),//#[source]は元エラーを保持する。エラーの連鎖を辿れるようにする。
    #[error("No rows affected: {0}")]
    NoRowsAffectedError(String),
    #[error("{0}")]
    KeyValueStoreError(#[from] redis::RedisError),
    #[error("{0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("{0}")]
    ConvertToUuidError(#[from] uuid::Error),
    #[error("ログインに失敗しました")]
    UnauthenticatedError,
    #[error("認可情報が誤っています")]
    UnauthorizedError,
    #[error("許可されていない操作です")]
    ForbiddenOperation,
    #[error("{0}")]
    ConversionEntityError(String),
}

impl IntoResponse for AppError {
    // AppErrorをHTTPレスポンスに変換する。
    fn into_response(self) -> axum::response::Response {
               let status_code = match self {
            AppError::UnprocessableEntity(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::EntityNotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) | AppError::ConvertToUuidError(_) => {
                StatusCode::BAD_REQUEST
            }
            AppError::UnauthenticatedError | AppError::ForbiddenOperation => StatusCode::FORBIDDEN,
            AppError::UnauthorizedError => StatusCode::UNAUTHORIZED,
            error @ (AppError::TransactionError(_) //パターンバインディング。|以降のいずれかに一致した場合の処理
            | AppError::SpecificOperationError(_)
            | AppError::NoRowsAffectedError(_)
            | AppError::KeyValueStoreError(_)
            | AppError::BcryptError(_)
            | AppError::ConversionEntityError(_)) => {
                tracing::error!(
                error.cause_chain = ?error,
                error.message = %error,
                "Unexpected error happened"
                );
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        status_code.into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
