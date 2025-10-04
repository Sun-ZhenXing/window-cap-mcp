use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::Arc;
use xcap::{Monitor, Window};

/// Python wrapper for monitor information
#[pyclass]
#[derive(Clone)]
pub struct PyMonitor {
    #[pyo3(get)]
    pub index: usize,
    #[pyo3(get)]
    pub name: String,
    #[pyo3(get)]
    pub x: i32,
    #[pyo3(get)]
    pub y: i32,
    #[pyo3(get)]
    pub width: u32,
    #[pyo3(get)]
    pub height: u32,
    #[pyo3(get)]
    pub is_primary: bool,
}

#[pymethods]
impl PyMonitor {
    fn __repr__(&self) -> String {
        format!(
            "PyMonitor(index={}, name='{}', size={}x{}, primary={})",
            self.index, self.name, self.width, self.height, self.is_primary
        )
    }

    fn to_dict(&self, py: Python) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        dict.set_item("index", self.index)?;
        dict.set_item("name", &self.name)?;
        dict.set_item("x", self.x)?;
        dict.set_item("y", self.y)?;
        dict.set_item("width", self.width)?;
        dict.set_item("height", self.height)?;
        dict.set_item("is_primary", self.is_primary)?;
        Ok(dict.into())
    }
}

/// Python wrapper for window information
#[pyclass]
#[derive(Clone)]
pub struct PyWindow {
    #[pyo3(get)]
    pub id: u32,
    #[pyo3(get)]
    pub title: String,
    #[pyo3(get)]
    pub app_name: String,
    #[pyo3(get)]
    pub x: i32,
    #[pyo3(get)]
    pub y: i32,
    #[pyo3(get)]
    pub width: u32,
    #[pyo3(get)]
    pub height: u32,
    #[pyo3(get)]
    pub is_minimized: bool,
    #[pyo3(get)]
    pub is_maximized: bool,
}

#[pymethods]
impl PyWindow {
    fn __repr__(&self) -> String {
        format!(
            "PyWindow(id={}, title='{}', app='{}', size={}x{})",
            self.id, self.title, self.app_name, self.width, self.height
        )
    }

    fn to_dict(&self, py: Python) -> PyResult<Py<PyAny>> {
        let dict = PyDict::new(py);
        dict.set_item("id", self.id)?;
        dict.set_item("title", &self.title)?;
        dict.set_item("app_name", &self.app_name)?;
        dict.set_item("x", self.x)?;
        dict.set_item("y", self.y)?;
        dict.set_item("width", self.width)?;
        dict.set_item("height", self.height)?;
        dict.set_item("is_minimized", self.is_minimized)?;
        dict.set_item("is_maximized", self.is_maximized)?;
        Ok(dict.into())
    }
}

/// Get the list of all monitors
#[pyfunction]
fn get_monitors() -> PyResult<Vec<PyMonitor>> {
    let monitors = Monitor::all().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get monitors: {}", e))
    })?;

    let result: Vec<PyMonitor> = monitors
        .iter()
        .enumerate()
        .map(|(idx, m)| PyMonitor {
            index: idx,
            name: m.name().unwrap_or_default(),
            x: m.x().unwrap_or(0),
            y: m.y().unwrap_or(0),
            width: m.width().unwrap_or(0),
            height: m.height().unwrap_or(0),
            is_primary: m.is_primary().unwrap_or(false),
        })
        .collect();

    Ok(result)
}

/// Get the count of monitors
#[pyfunction]
fn get_monitor_count() -> PyResult<usize> {
    let monitors = Monitor::all().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get monitors: {}", e))
    })?;
    Ok(monitors.len())
}

/// Capture a screenshot from the specified monitor
///
/// Args:
///     monitor_index: Optional monitor index. If None, captures from primary monitor.
///
/// Returns:
///     Base64 encoded PNG image
#[pyfunction]
fn capture_monitor(monitor_index: Option<usize>) -> PyResult<String> {
    let monitors = Monitor::all().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get monitors: {}", e))
    })?;

    if monitors.is_empty() {
        return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
            "No monitors available",
        ));
    }

    let monitor = if let Some(idx) = monitor_index {
        monitors.get(idx).ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyIndexError, _>(format!(
                "Monitor index {} does not exist",
                idx
            ))
        })?
    } else {
        monitors
            .iter()
            .find(|m| m.is_primary().unwrap_or(false))
            .or_else(|| monitors.first())
            .ok_or_else(|| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Unable to find primary monitor")
            })?
    };

    let image = monitor.capture_image().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Screenshot failed: {}", e))
    })?;

    let mut buffer = Vec::new();
    image
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Image encoding failed: {}",
                e
            ))
        })?;

    let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, buffer);
    Ok(base64_image)
}

/// Get the list of all windows
#[pyfunction]
fn get_windows() -> PyResult<Vec<PyWindow>> {
    let windows = Window::all().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get windows: {}", e))
    })?;

    let result: Vec<PyWindow> = windows
        .iter()
        .map(|w| PyWindow {
            id: w.id().unwrap_or(0),
            title: w.title().unwrap_or_default(),
            app_name: w.app_name().unwrap_or_default(),
            x: w.x().unwrap_or(0),
            y: w.y().unwrap_or(0),
            width: w.width().unwrap_or(0),
            height: w.height().unwrap_or(0),
            is_minimized: w.is_minimized().unwrap_or(false),
            is_maximized: w.is_maximized().unwrap_or(false),
        })
        .collect();

    Ok(result)
}

/// Get the count of windows
#[pyfunction]
fn get_window_count() -> PyResult<usize> {
    let windows = Window::all().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get windows: {}", e))
    })?;
    Ok(windows.len())
}

/// Capture a screenshot from the specified window
///
/// Args:
///     window_id: Window ID
///
/// Returns:
///     Base64 encoded PNG image
#[pyfunction]
fn capture_window(window_id: u32) -> PyResult<String> {
    let windows = Window::all().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get windows: {}", e))
    })?;

    let window = windows
        .iter()
        .find(|w| w.id().unwrap_or(0) == window_id)
        .ok_or_else(|| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Window with ID {} not found",
                window_id
            ))
        })?;

    let image = window.capture_image().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Screenshot failed: {}", e))
    })?;

    let mut buffer = Vec::new();
    image
        .write_to(
            &mut std::io::Cursor::new(&mut buffer),
            image::ImageFormat::Png,
        )
        .map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Image encoding failed: {}",
                e
            ))
        })?;

    let base64_image = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, buffer);
    Ok(base64_image)
}

/// Run the MCP server with specified options
///
/// Args:
///     sse (bool): Use SSE (Server-Sent Events) protocol
///     http (bool): Use Streamable HTTP protocol
///     port (int): Port to listen on (for HTTP/SSE mode)
///     host (str): Host to bind to (for HTTP/SSE mode)
///
/// Examples:
///     >>> import window_cap_mcp as wc
///     >>> # Run in STDIO mode (default)
///     >>> wc.run_server()
///     >>>
///     >>> # Run in SSE mode
///     >>> wc.run_server(sse=True, port=8080)
///     >>>
///     >>> # Run in HTTP mode
///     >>> wc.run_server(http=True, port=3000, host="0.0.0.0")
#[pyfunction]
#[pyo3(signature = (sse=false, http=false, port=8080, host="127.0.0.1".to_string()))]
fn run_server(py: Python, sse: bool, http: bool, port: u16, host: String) -> PyResult<()> {
    use crate::handler::WindowCapServer;
    use rmcp::{
        transport::{
            sse_server::SseServer,
            stdio,
            streamable_http_server::{
                session::local::LocalSessionManager,
                tower::{StreamableHttpServerConfig, StreamableHttpService},
            },
        },
        ServiceExt,
    };
    use std::net::SocketAddr;

    // Release GIL for async operations
    py.detach(|| {
        // Create a new Tokio runtime
        let rt = tokio::runtime::Runtime::new().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                "Failed to create Tokio runtime: {}",
                e
            ))
        })?;

        rt.block_on(async {
            if sse {
                eprintln!("Starting server in SSE mode...");
                let addr: SocketAddr = format!("{}:{}", host, port).parse().map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid address: {}",
                        e
                    ))
                })?;
                eprintln!("Binding to: {}", addr);

                let ct = SseServer::serve(addr)
                    .await
                    .map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to start SSE server: {}",
                            e
                        ))
                    })?
                    .with_service(WindowCapServer::new);

                eprintln!("SSE server started, visit http://{}", addr);
                tokio::signal::ctrl_c().await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to wait for Ctrl+C: {}",
                        e
                    ))
                })?;
                ct.cancel();
                eprintln!("Server stopped");
            } else if http {
                eprintln!("Starting server in HTTP Streamable mode...");
                let addr: SocketAddr = format!("{}:{}", host, port).parse().map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid address: {}",
                        e
                    ))
                })?;
                eprintln!("Binding to: {}", addr);

                let session_manager = Arc::new(LocalSessionManager::default());
                let service_factory = || Ok(WindowCapServer::new());
                let config = StreamableHttpServerConfig::default();
                let http_service =
                    StreamableHttpService::new(service_factory, session_manager, config);
                let listener = tokio::net::TcpListener::bind(addr).await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to bind to address: {}",
                        e
                    ))
                })?;
                eprintln!("HTTP Streamable server started at http://{}", addr);

                loop {
                    let (stream, peer_addr) = listener.accept().await.map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                            "Failed to accept connection: {}",
                            e
                        ))
                    })?;
                    eprintln!("Accepted connection from: {}", peer_addr);
                    let http_service = http_service.clone();
                    tokio::spawn(async move {
                        use hyper_util::rt::TokioIo;
                        use hyper_util::server::conn::auto::Builder;
                        use tower::Service;
                        let io = TokioIo::new(stream);
                        let service = http_service.clone();
                        let hyper_service = hyper::service::service_fn(move |req| {
                            let mut svc = service.clone();
                            async move { svc.call(req).await }
                        });
                        if let Err(e) = Builder::new(hyper_util::rt::TokioExecutor::new())
                            .serve_connection(io, hyper_service)
                            .await
                        {
                            eprintln!("Error serving connection: {}", e);
                        }
                    });
                }
            } else {
                eprintln!("Starting server in STDIO mode...");
                let server = WindowCapServer::new();
                let service = server.serve(stdio()).await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to start STDIO server: {}",
                        e
                    ))
                })?;
                service.waiting().await.map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Server error: {}",
                        e
                    ))
                })?;
            }
            Ok::<(), PyErr>(())
        })
    })
}

/// Window Capture MCP - Python bindings
///
/// A cross-platform window capture library for Python.
///
/// Functions:
///     get_monitors() -> List[PyMonitor]: Get list of all monitors
///     get_monitor_count() -> int: Get count of monitors
///     capture_monitor(monitor_index: Optional[int] = None) -> str: Capture screenshot from monitor (returns base64 PNG)
///     get_windows() -> List[PyWindow]: Get list of all windows
///     get_window_count() -> int: Get count of windows
///     capture_window(window_id: int) -> str: Capture screenshot from window (returns base64 PNG)
///     run_server(sse: bool = False, http: bool = False, port: int = 8080, host: str = "127.0.0.1") -> None: Run MCP server
#[pymodule]
fn window_cap_mcp(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_monitors, m)?)?;
    m.add_function(wrap_pyfunction!(get_monitor_count, m)?)?;
    m.add_function(wrap_pyfunction!(capture_monitor, m)?)?;
    m.add_function(wrap_pyfunction!(get_windows, m)?)?;
    m.add_function(wrap_pyfunction!(get_window_count, m)?)?;
    m.add_function(wrap_pyfunction!(capture_window, m)?)?;
    m.add_function(wrap_pyfunction!(run_server, m)?)?;
    m.add_class::<PyMonitor>()?;
    m.add_class::<PyWindow>()?;
    Ok(())
}
