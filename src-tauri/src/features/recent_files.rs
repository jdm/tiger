use tauri::Manager;

use crate::app::AppState;
use crate::utils::observable::Observer;

pub fn init(tauri_app: &tauri::App) {
    let app_state = tauri_app.state::<AppState>();
    let app = app_state.0.lock();
    app.recent_documents_delegate()
        .subscribe(|recent_documents| {
            dbg!(&recent_documents);
            Observer::StaySubscribed
        });
}
