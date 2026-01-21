use block_dn_client::{Builder, Client};

fn default_client() -> Client<'static> {
    Builder::default().build()
}

#[test]
fn test_html() {
    let client = default_client();
    assert!(client.index_html().is_ok());
}

#[test]
fn test_status() {
    let client = default_client();
    assert!(client.status().is_ok());
}

#[test]
fn test_headers() {
    let client = default_client();
    assert!(client.block_headers(0).is_ok());
}

#[test]
fn test_filters() {
    let client = default_client();
    let filters = client.filters(0).unwrap();
    assert!(filters.len() == 2000);
}
