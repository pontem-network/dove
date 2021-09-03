pub fn api_url() -> String {
    format!("{}/api/rpc", web_sys::window()
        .and_then(|win| win.location().origin().ok())
        .unwrap_or_default())
}
