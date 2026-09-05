use crate::presentation::tui::app::TUIApp;
use crate::shared::kernel::result::AppResult;

pub(crate) async fn run_tui() -> AppResult<()> {
    let mut app = TUIApp::new().await?;
    app.run().await
}
