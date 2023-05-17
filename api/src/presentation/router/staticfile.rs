//! 静的ファイルのルーティング
//!
//! 他のルータのマッチにしない場合(fallback)、`public`フォルダから対象ファイルを返す。

use axum::{http::StatusCode, routing::*, Router};
use tower_http::services::ServeDir;

pub fn static_roouter() -> Router {
    // 参考　https://stackoverflow.com/questions/73464479/how-to-host-spa-files-and-embed-too-with-axum-and-rust-embed
    let static_files_service = get_service(
        ServeDir::new("public").append_index_html_on_directories(true),
    )
    .handle_error(|error: std::io::Error| async move {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Unhandled internal error: {}", error),
        )
    });

    Router::new().fallback(static_files_service)
}
