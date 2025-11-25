use reqwest::Client;
use std::sync::OnceLock;

static CLIENT: OnceLock<Client> = OnceLock::new();

// similar to interceptor in axios
pub fn get_client() -> &'static Client {
    CLIENT.get_or_init(|| {
        Client::builder()
            .cookie_store(true)
            .build()
            .expect("Failed to create HTTP client")
    })
}
