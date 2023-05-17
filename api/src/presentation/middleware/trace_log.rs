//! アクセス解析ログ ミドルウェア
//!
//! 各リクエストとレスポンスのアクセスを監視します。

use tower_http::{
    classify::{ServerErrorsAsFailures, SharedClassifier},
    trace,
    trace::TraceLayer,
};

pub fn trace_middleware() -> TraceLayer<SharedClassifier<ServerErrorsAsFailures>> {
    TraceLayer::new_for_http()
        .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
        .on_request(trace::DefaultOnRequest::new().level(tracing::Level::INFO))
        .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO))
        .on_body_chunk(trace::DefaultOnBodyChunk::new())
        .on_eos(trace::DefaultOnEos::new().level(tracing::Level::INFO))
        .on_failure(trace::DefaultOnFailure::new().level(tracing::Level::INFO))
}
