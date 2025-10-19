use crate::models::*;
use crate::utils::window_ops;
use rmcp::{
    handler::server::wrapper::Parameters, model::*, tool, tool_handler, tool_router,
    ErrorData as McpError, ServerHandler,
};
use xcap::{Monitor, Window};

#[derive(Clone)]
pub struct WindowCapServer {
    pub tool_router: rmcp::handler::server::tool::ToolRouter<Self>,
}

impl Default for WindowCapServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl WindowCapServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get the number and details of monitors")]
    async fn get_monitor_count(
        &self,
        _params: Parameters<EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        // 在阻塞线程中执行获取显示器信息的操作
        let result = tokio::task::spawn_blocking(move || -> Result<String, String> {
            let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;

            let monitor_list: Vec<_> = monitors
                .iter()
                .enumerate()
                .map(|(idx, m)| {
                    serde_json::json!({
                        "index": idx,
                        "name": m.name().unwrap_or_default(),
                        "x": m.x().unwrap_or(0),
                        "y": m.y().unwrap_or(0),
                        "width": m.width().unwrap_or(0),
                        "height": m.height().unwrap_or(0),
                        "is_primary": m.is_primary().unwrap_or(false),
                    })
                })
                .collect();

            let result = serde_json::json!({
                "count": monitor_list.len(),
                "monitors": monitor_list
            });

            serde_json::to_string_pretty(&result)
                .map_err(|e| format!("JSON serialization failed: {}", e))
        })
        .await
        .map_err(|e| McpError::internal_error(format!("Task join error: {}", e), None))?
        .map_err(|e| McpError::internal_error(e, None))?;

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get a screenshot of the specified monitor")]
    async fn get_screen_screenshot(
        &self,
        params: Parameters<ScreenshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let monitor_index = params.0.monitor_index.map(|idx| idx as usize);

        // 在阻塞线程中执行耗时的截图和编码操作
        let result = tokio::task::spawn_blocking(move || -> Result<(String, String), String> {
            let monitors = Monitor::all().map_err(|e| format!("Failed to get monitors: {}", e))?;

            if monitors.is_empty() {
                return Err("No monitors available".to_string());
            }

            let monitor = if let Some(idx) = monitor_index {
                monitors
                    .get(idx)
                    .ok_or_else(|| format!("Monitor index {} does not exist", idx))?
            } else {
                monitors
                    .iter()
                    .find(|m| m.is_primary().unwrap_or(false))
                    .or_else(|| monitors.first())
                    .ok_or_else(|| "Unable to find primary monitor".to_string())?
            };

            let image = monitor
                .capture_image()
                .map_err(|e| format!("Screenshot failed: {}", e))?;

            let mut buffer = Vec::new();
            image
                .write_to(
                    &mut std::io::Cursor::new(&mut buffer),
                    image::ImageFormat::Png,
                )
                .map_err(|e| format!("Image encoding failed: {}", e))?;

            let base64_image =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, buffer);

            let metadata = format!(
                "Monitor: {} (Index: {}, Size: {}x{})",
                monitor.name().unwrap_or_default(),
                monitor_index.unwrap_or(0),
                monitor.width().unwrap_or(0),
                monitor.height().unwrap_or(0)
            );

            Ok((metadata, base64_image))
        })
        .await
        .map_err(|e| McpError::internal_error(format!("Task join error: {}", e), None))?
        .map_err(|e| McpError::internal_error(e, None))?;

        Ok(CallToolResult::success(vec![
            Content::text(result.0),
            Content::image(result.1, "image/png".to_string()),
        ]))
    }

    #[tool(description = "Get a list of all windows")]
    async fn get_window_list(
        &self,
        _params: Parameters<EmptyParams>,
    ) -> Result<CallToolResult, McpError> {
        // 在阻塞线程中执行获取窗口列表的操作
        let result = tokio::task::spawn_blocking(move || -> Result<String, String> {
            let windows = Window::all().map_err(|e| format!("Failed to get window list: {}", e))?;

            let window_list: Vec<_> = windows
                .iter()
                .map(|w| {
                    serde_json::json!({
                        "id": w.id().unwrap_or(0),
                        "title": w.title().unwrap_or_default(),
                        "app_name": w.app_name().unwrap_or_default(),
                        "x": w.x().unwrap_or(0),
                        "y": w.y().unwrap_or(0),
                        "width": w.width().unwrap_or(0),
                        "height": w.height().unwrap_or(0),
                        "is_minimized": w.is_minimized().unwrap_or(false),
                        "is_maximized": w.is_maximized().unwrap_or(false),
                    })
                })
                .collect();

            let result = serde_json::json!({
                "count": window_list.len(),
                "windows": window_list
            });

            serde_json::to_string_pretty(&result)
                .map_err(|e| format!("JSON serialization failed: {}", e))
        })
        .await
        .map_err(|e| McpError::internal_error(format!("Task join error: {}", e), None))?
        .map_err(|e| McpError::internal_error(e, None))?;

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }

    #[tool(description = "Get a screenshot of the specified window")]
    async fn get_window_screenshot(
        &self,
        params: Parameters<WindowScreenshotParams>,
    ) -> Result<CallToolResult, McpError> {
        let window_id = params.0.window_id;

        // 在阻塞线程中执行耗时的截图和编码操作
        let result = tokio::task::spawn_blocking(move || -> Result<(String, String), String> {
            let windows = Window::all().map_err(|e| format!("Failed to get window list: {}", e))?;

            let window = windows
                .iter()
                .find(|w| w.id().unwrap_or(0) == window_id)
                .ok_or_else(|| format!("Window ID {} does not exist", window_id))?;

            let image = window
                .capture_image()
                .map_err(|e| format!("Window screenshot failed: {}", e))?;

            let mut buffer = Vec::new();
            image
                .write_to(
                    &mut std::io::Cursor::new(&mut buffer),
                    image::ImageFormat::Png,
                )
                .map_err(|e| format!("Image encoding failed: {}", e))?;

            let base64_image =
                base64::Engine::encode(&base64::engine::general_purpose::STANDARD, buffer);

            let metadata = format!(
                "Window: {} [{}] (ID: {}, Size: {}x{})",
                window.title().unwrap_or_default(),
                window.app_name().unwrap_or_default(),
                window.id().unwrap_or(0),
                window.width().unwrap_or(0),
                window.height().unwrap_or(0)
            );

            Ok((metadata, base64_image))
        })
        .await
        .map_err(|e| McpError::internal_error(format!("Task join error: {}", e), None))?
        .map_err(|e| McpError::internal_error(e, None))?;

        Ok(CallToolResult::success(vec![
            Content::text(result.0),
            Content::image(result.1, "image/png".to_string()),
        ]))
    }

    #[tool(description = "Close a window")]
    async fn close_window(
        &self,
        params: Parameters<CloseWindowParams>,
    ) -> Result<CallToolResult, McpError> {
        let window_id = params.0.window_id;

        // Perform the close window operation in a blocking thread
        let result =
            tokio::task::spawn_blocking(move || window_ops::close_window_with_info(window_id))
                .await
                .map_err(|e| McpError::internal_error(format!("Task join error: {}", e), None))?
                .map_err(|e| McpError::internal_error(e, None))?;

        Ok(CallToolResult::success(vec![Content::text(result)]))
    }
}

#[tool_handler]
impl ServerHandler for WindowCapServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: Implementation {
                name: "window-cap-mcp".to_string(),
                version: "0.2.0".to_string(),
                title: None,
                icons: None,
                website_url: None,
            },
            instructions: Some(
                "Cross-platform window and screen screenshot MCP server.".to_string(),
            ),
        }
    }
}
