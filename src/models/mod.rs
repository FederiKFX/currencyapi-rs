use std::collections::HashMap;

/// Response of the currencyapi
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct DetailsResponse {
    /// Data source
    pub data: HashMap<String, f64>,
}
