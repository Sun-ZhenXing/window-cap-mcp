use rmcp::schemars;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct EmptyParams {}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct ScreenshotParams {
    /// Monitor index, uses primary monitor if not specified
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monitor_index: Option<u32>,
}

#[derive(Serialize, Deserialize, schemars::JsonSchema)]
pub struct WindowScreenshotParams {
    /// Window ID
    pub window_id: u32,
}
