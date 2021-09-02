use once_cell::sync::OnceCell;

static INSTANCE: OnceCell<State> = OnceCell::new();

#[derive(Debug)]
struct State {
    api_url: String,
}

pub fn set_base_url(url: &str) {
    INSTANCE.get_or_init(|| State {
        api_url: format!("{}/api/rpc", url)
    });
}

pub fn api_url() -> &'static str {
    if let Some(state) = INSTANCE.get() {
        state.api_url.as_str()
    } else {
        ""
    }
}