pub mod geometry;  // ジオメトリ関連
pub mod widget;    // ウィジェット定義
pub mod app_state; // アプリ状態
pub mod run;       // 実行エントリ

/// 同期 main から非同期ランループ `run::run` を起動するエントリポイント。
fn main() { pollster::block_on(run::run()); }
