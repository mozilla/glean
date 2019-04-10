pub enum Lifetime {
    /// The metric is reset with each sent ping
    Ping,
    /// The metric is reset on application restart
    Application,
    /// The metric is reset with each user profile
    User
}

pub struct CommonMetricData {
    pub name: String,
    pub category: String,
    pub send_in_pings: Vec<String>,
    pub lifetime: Lifetime,
    pub disabled: bool,
}
