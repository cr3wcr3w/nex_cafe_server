use crate::shared::constants::PROJECTNAME;

pub fn setup_webview2_user_data() {
    if std::env::var_os("WEBVIEW2_USER_DATA_FOLDER").is_none() {
        if let Some(dir) = dirs::data_local_dir() {
            let path = dir.join(PROJECTNAME).join("WebView2");
            let _ = std::fs::create_dir_all(&path);
            std::env::set_var("WEBVIEW2_USER_DATA_FOLDER", &path);
        }
    }
}
