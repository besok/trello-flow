pub struct Settings {
    pub credentials_path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            credentials_path: "/usr/local/share/appdata/cred.json".to_string(),
        }
    }
}
