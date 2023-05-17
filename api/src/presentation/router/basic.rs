//! Query パラメータ テスト
//!  
//! Queryに応じたレスポンスを返します。

use super::super::handler::basic::*;

use axum::{routing::*, Router};

pub fn basic() -> Router {
    Router::new().route("/prime", get(search))
}
