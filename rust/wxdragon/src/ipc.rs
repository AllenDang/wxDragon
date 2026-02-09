//! IPC (Inter-Process Communication) module for wxDragon.
//!
//! This module provides safe wrappers around wxWidgets' generic IPC classes (wxServer,
//! wxClient, wxConnection). The underlying transport is platform-dependent:
//! - **Windows**: Uses DDE (Dynamic Data Exchange), which is OS-native and does not
//!   trigger firewall prompts.
//! - **Unix/macOS**: Uses TCP sockets. Unix domain sockets are also supported when a
//!   file path is passed as the service name instead of a port number.
//!
//! # Overview
//!
//! wxIPC provides a simple client-server model:
//! - A **Server** listens on a service (port) and accepts connections
//! - A **Client** connects to a server on a host/service/topic
//! - A **Connection** is established between client and server for data exchange
//!
//! # Example
//!
//! ```rust,no_run
//! use wxdragon::ipc::{IPCServer, IPCClient, IPCConnection, IPCFormat};
//!
//! // Server side
//! let server = IPCServer::new(|topic| {
//!     println!("Client connecting to topic: {}", topic);
//!     Some(IPCConnection::builder()
//!         .on_execute(|_topic, data, _format| {
//!             println!("Received execute: {:?}", data);
//!             true
//!         })
//!         .build())
//! });
//! server.create("4242"); // Listen on port 4242
//!
//! // Client side
//! let client = IPCClient::new();
//! if let Some(conn) = client.make_connection("localhost", "4242", "test") {
//!     conn.execute_string("Hello, server!");
//! }
//! ```

use std::ffi::{CStr, CString};
use std::os::raw::c_void;
use std::ptr;
use wxdragon_sys as ffi;

/// IPC data format for Execute, Request, Poke, and Advise operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum IPCFormat {
    /// Text format (CF_TEXT)
    Text = 1,
    /// Bitmap format (CF_BITMAP)
    Bitmap = 2,
    /// Metafile format (CF_METAFILEPICT)
    Metafile = 3,
    /// Unicode text format (CF_UNICODETEXT)
    UnicodeText = 13,
    /// UTF-8 text format
    Utf8Text = 14,
    /// Private/binary data format
    Private = 20,
}

impl From<ffi::wxd_IPCFormat> for IPCFormat {
    fn from(format: ffi::wxd_IPCFormat) -> Self {
        match format {
            ffi::wxd_IPCFormat_WXD_IPC_TEXT => IPCFormat::Text,
            ffi::wxd_IPCFormat_WXD_IPC_BITMAP => IPCFormat::Bitmap,
            ffi::wxd_IPCFormat_WXD_IPC_METAFILE => IPCFormat::Metafile,
            ffi::wxd_IPCFormat_WXD_IPC_UNICODETEXT => IPCFormat::UnicodeText,
            ffi::wxd_IPCFormat_WXD_IPC_UTF8TEXT => IPCFormat::Utf8Text,
            ffi::wxd_IPCFormat_WXD_IPC_PRIVATE => IPCFormat::Private,
            _ => IPCFormat::Text,
        }
    }
}

impl From<IPCFormat> for ffi::wxd_IPCFormat {
    fn from(format: IPCFormat) -> Self {
        match format {
            IPCFormat::Text => ffi::wxd_IPCFormat_WXD_IPC_TEXT,
            IPCFormat::Bitmap => ffi::wxd_IPCFormat_WXD_IPC_BITMAP,
            IPCFormat::Metafile => ffi::wxd_IPCFormat_WXD_IPC_METAFILE,
            IPCFormat::UnicodeText => ffi::wxd_IPCFormat_WXD_IPC_UNICODETEXT,
            IPCFormat::Utf8Text => ffi::wxd_IPCFormat_WXD_IPC_UTF8TEXT,
            IPCFormat::Private => ffi::wxd_IPCFormat_WXD_IPC_PRIVATE,
        }
    }
}

// =============================================================================
// Connection Callbacks - stored in a Box and passed to C++
// =============================================================================

// Type aliases for callback types to reduce complexity warnings
type ExecuteCallback = Box<dyn FnMut(&str, &[u8], IPCFormat) -> bool>;
type RequestCallback = Box<dyn FnMut(&str, &str, IPCFormat) -> Option<Vec<u8>>>;
type PokeCallback = Box<dyn FnMut(&str, &str, &[u8], IPCFormat) -> bool>;
type AdviseTopicCallback = Box<dyn FnMut(&str, &str) -> bool>;
type AdviseDataCallback = Box<dyn FnMut(&str, &str, &[u8], IPCFormat) -> bool>;
type DisconnectCallback = Box<dyn FnMut() -> bool>;
type AcceptConnectionCallback = Box<dyn FnMut(&str) -> Option<IPCConnection>>;

/// Internal structure holding all connection callbacks.
struct ConnectionCallbacks {
    on_execute: Option<ExecuteCallback>,
    on_request: Option<RequestCallback>,
    on_poke: Option<PokeCallback>,
    on_start_advise: Option<AdviseTopicCallback>,
    on_stop_advise: Option<AdviseTopicCallback>,
    on_advise: Option<AdviseDataCallback>,
    on_disconnect: Option<DisconnectCallback>,
    /// Buffer for OnRequest response data (must outlive the callback return)
    request_buffer: Vec<u8>,
}

impl ConnectionCallbacks {
    fn new() -> Self {
        Self {
            on_execute: None,
            on_request: None,
            on_poke: None,
            on_start_advise: None,
            on_stop_advise: None,
            on_advise: None,
            on_disconnect: None,
            request_buffer: Vec::new(),
        }
    }
}

// =============================================================================
// C Callback Trampolines
// =============================================================================

// Allow unsafe operations in unsafe fns for cleaner trampoline code
#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_execute_trampoline(
    user_data: *mut c_void,
    topic: *const i8,
    data: *const c_void,
    size: usize,
    format: ffi::wxd_IPCFormat,
) -> bool {
    if user_data.is_null() {
        return false;
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_execute {
        let topic_str = if topic.is_null() {
            ""
        } else {
            CStr::from_ptr(topic).to_str().unwrap_or("")
        };
        let data_slice = if data.is_null() || size == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(data as *const u8, size)
        };
        return cb(topic_str, data_slice, IPCFormat::from(format));
    }
    false
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_request_trampoline(
    user_data: *mut c_void,
    topic: *const i8,
    item: *const i8,
    out_size: *mut usize,
    format: ffi::wxd_IPCFormat,
) -> *const c_void {
    if user_data.is_null() {
        return ptr::null();
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_request {
        let topic_str = if topic.is_null() {
            ""
        } else {
            CStr::from_ptr(topic).to_str().unwrap_or("")
        };
        let item_str = if item.is_null() {
            ""
        } else {
            CStr::from_ptr(item).to_str().unwrap_or("")
        };
        if let Some(data) = cb(topic_str, item_str, IPCFormat::from(format)) {
            // Store in buffer so it outlives this function
            callbacks.request_buffer = data;
            if !out_size.is_null() {
                *out_size = callbacks.request_buffer.len();
            }
            return callbacks.request_buffer.as_ptr() as *const c_void;
        }
    }
    if !out_size.is_null() {
        *out_size = 0;
    }
    ptr::null()
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_poke_trampoline(
    user_data: *mut c_void,
    topic: *const i8,
    item: *const i8,
    data: *const c_void,
    size: usize,
    format: ffi::wxd_IPCFormat,
) -> bool {
    if user_data.is_null() {
        return false;
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_poke {
        let topic_str = if topic.is_null() {
            ""
        } else {
            CStr::from_ptr(topic).to_str().unwrap_or("")
        };
        let item_str = if item.is_null() {
            ""
        } else {
            CStr::from_ptr(item).to_str().unwrap_or("")
        };
        let data_slice = if data.is_null() || size == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(data as *const u8, size)
        };
        return cb(topic_str, item_str, data_slice, IPCFormat::from(format));
    }
    false
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_start_advise_trampoline(user_data: *mut c_void, topic: *const i8, item: *const i8) -> bool {
    if user_data.is_null() {
        return false;
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_start_advise {
        let topic_str = if topic.is_null() {
            ""
        } else {
            CStr::from_ptr(topic).to_str().unwrap_or("")
        };
        let item_str = if item.is_null() {
            ""
        } else {
            CStr::from_ptr(item).to_str().unwrap_or("")
        };
        return cb(topic_str, item_str);
    }
    false
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_stop_advise_trampoline(user_data: *mut c_void, topic: *const i8, item: *const i8) -> bool {
    if user_data.is_null() {
        return false;
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_stop_advise {
        let topic_str = if topic.is_null() {
            ""
        } else {
            CStr::from_ptr(topic).to_str().unwrap_or("")
        };
        let item_str = if item.is_null() {
            ""
        } else {
            CStr::from_ptr(item).to_str().unwrap_or("")
        };
        return cb(topic_str, item_str);
    }
    false
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_advise_trampoline(
    user_data: *mut c_void,
    topic: *const i8,
    item: *const i8,
    data: *const c_void,
    size: usize,
    format: ffi::wxd_IPCFormat,
) -> bool {
    if user_data.is_null() {
        return false;
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_advise {
        let topic_str = if topic.is_null() {
            ""
        } else {
            CStr::from_ptr(topic).to_str().unwrap_or("")
        };
        let item_str = if item.is_null() {
            ""
        } else {
            CStr::from_ptr(item).to_str().unwrap_or("")
        };
        let data_slice = if data.is_null() || size == 0 {
            &[]
        } else {
            std::slice::from_raw_parts(data as *const u8, size)
        };
        return cb(topic_str, item_str, data_slice, IPCFormat::from(format));
    }
    false
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_disconnect_trampoline(user_data: *mut c_void) -> bool {
    if user_data.is_null() {
        return true;
    }
    let callbacks = &mut *(user_data as *mut ConnectionCallbacks);
    if let Some(ref mut cb) = callbacks.on_disconnect {
        return cb();
    }
    true
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn free_connection_callbacks(user_data: *mut c_void) {
    if !user_data.is_null() {
        let _ = Box::from_raw(user_data as *mut ConnectionCallbacks);
    }
}

// =============================================================================
// IPCConnection
// =============================================================================

/// A connection between an IPC client and server.
///
/// Connections are used to exchange data using Execute, Request, Poke, and Advise
/// operations. The connection can be created by the server (in OnAcceptConnection)
/// or returned from a client's MakeConnection call.
pub struct IPCConnection {
    ptr: *mut ffi::wxd_IPCConnection_t,
    /// Whether we own the pointer and should destroy it
    owned: bool,
}

impl IPCConnection {
    /// Create a new connection builder.
    pub fn builder() -> IPCConnectionBuilder {
        IPCConnectionBuilder::new()
    }

    /// Create a connection from a raw pointer (used internally).
    ///
    /// # Safety
    /// The pointer must be valid and the caller transfers ownership.
    #[allow(dead_code)]
    pub(crate) unsafe fn from_ptr(ptr: *mut ffi::wxd_IPCConnection_t) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self { ptr, owned: false })
        }
    }

    /// Get the raw pointer (for internal use).
    #[allow(dead_code)]
    pub(crate) fn as_ptr(&self) -> *mut ffi::wxd_IPCConnection_t {
        self.ptr
    }

    /// Execute a command on the remote side.
    ///
    /// On the server side, this triggers the client's OnExecute callback.
    /// On the client side, this triggers the server's OnExecute callback.
    pub fn execute(&self, data: &[u8], format: IPCFormat) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_IPCConnection_Execute(self.ptr, data.as_ptr() as *const c_void, data.len(), format.into()) }
    }

    /// Execute a string command (convenience method for text data).
    pub fn execute_string(&self, data: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_str = match CString::new(data) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_IPCConnection_ExecuteString(self.ptr, c_str.as_ptr()) }
    }

    /// Request data from the remote side.
    ///
    /// Returns the data if the request was successful, None otherwise.
    pub fn request(&self, item: &str, format: IPCFormat) -> Option<Vec<u8>> {
        if self.ptr.is_null() {
            return None;
        }
        let c_item = CString::new(item).ok()?;
        let mut size: usize = 0;
        let data_ptr = unsafe { ffi::wxd_IPCConnection_Request(self.ptr, c_item.as_ptr(), &mut size, format.into()) };
        if data_ptr.is_null() || size == 0 {
            return None;
        }
        let data_slice = unsafe { std::slice::from_raw_parts(data_ptr as *const u8, size) };
        Some(data_slice.to_vec())
    }

    /// Poke data to the remote side.
    pub fn poke(&self, item: &str, data: &[u8], format: IPCFormat) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_item = match CString::new(item) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe {
            ffi::wxd_IPCConnection_Poke(
                self.ptr,
                c_item.as_ptr(),
                data.as_ptr() as *const c_void,
                data.len(),
                format.into(),
            )
        }
    }

    /// Start an advise loop for the given item.
    ///
    /// The server will send updates via the OnAdvise callback when the item changes.
    pub fn start_advise(&self, item: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_item = match CString::new(item) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_IPCConnection_StartAdvise(self.ptr, c_item.as_ptr()) }
    }

    /// Stop an advise loop for the given item.
    pub fn stop_advise(&self, item: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_item = match CString::new(item) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_IPCConnection_StopAdvise(self.ptr, c_item.as_ptr()) }
    }

    /// Send advised data to the client (server-side only).
    pub fn advise(&self, item: &str, data: &[u8], format: IPCFormat) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_item = match CString::new(item) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe {
            ffi::wxd_IPCConnection_Advise(
                self.ptr,
                c_item.as_ptr(),
                data.as_ptr() as *const c_void,
                data.len(),
                format.into(),
            )
        }
    }

    /// Disconnect the connection.
    pub fn disconnect(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_IPCConnection_Disconnect(self.ptr) }
    }

    /// Check if the connection is still connected.
    pub fn is_connected(&self) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        unsafe { ffi::wxd_IPCConnection_IsConnected(self.ptr) }
    }
}

impl Drop for IPCConnection {
    fn drop(&mut self) {
        if self.owned && !self.ptr.is_null() {
            unsafe { ffi::wxd_IPCConnection_Destroy(self.ptr) };
        }
    }
}

/// Builder for creating an IPCConnection with callbacks.
pub struct IPCConnectionBuilder {
    callbacks: ConnectionCallbacks,
}

impl IPCConnectionBuilder {
    /// Create a new connection builder.
    pub fn new() -> Self {
        Self {
            callbacks: ConnectionCallbacks::new(),
        }
    }

    /// Set the OnExecute callback (server-side: called when client executes a command).
    pub fn on_execute<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &[u8], IPCFormat) -> bool + 'static,
    {
        self.callbacks.on_execute = Some(Box::new(callback));
        self
    }

    /// Set the OnRequest callback (server-side: called when client requests data).
    pub fn on_request<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str, IPCFormat) -> Option<Vec<u8>> + 'static,
    {
        self.callbacks.on_request = Some(Box::new(callback));
        self
    }

    /// Set the OnPoke callback (server-side: called when client pokes data).
    pub fn on_poke<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str, &[u8], IPCFormat) -> bool + 'static,
    {
        self.callbacks.on_poke = Some(Box::new(callback));
        self
    }

    /// Set the OnStartAdvise callback (server-side: called when client starts advise).
    pub fn on_start_advise<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str) -> bool + 'static,
    {
        self.callbacks.on_start_advise = Some(Box::new(callback));
        self
    }

    /// Set the OnStopAdvise callback (server-side: called when client stops advise).
    pub fn on_stop_advise<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str) -> bool + 'static,
    {
        self.callbacks.on_stop_advise = Some(Box::new(callback));
        self
    }

    /// Set the OnAdvise callback (client-side: called when server sends advised data).
    pub fn on_advise<F>(mut self, callback: F) -> Self
    where
        F: FnMut(&str, &str, &[u8], IPCFormat) -> bool + 'static,
    {
        self.callbacks.on_advise = Some(Box::new(callback));
        self
    }

    /// Set the OnDisconnect callback (both sides: called when connection is terminated).
    pub fn on_disconnect<F>(mut self, callback: F) -> Self
    where
        F: FnMut() -> bool + 'static,
    {
        self.callbacks.on_disconnect = Some(Box::new(callback));
        self
    }

    /// Build the connection.
    pub fn build(self) -> IPCConnection {
        let callbacks_box = Box::new(self.callbacks);
        let user_data = Box::into_raw(callbacks_box) as *mut c_void;

        let ptr = unsafe {
            ffi::wxd_IPCConnection_Create(
                user_data,
                Some(on_execute_trampoline),
                Some(on_request_trampoline),
                Some(on_poke_trampoline),
                Some(on_start_advise_trampoline),
                Some(on_stop_advise_trampoline),
                Some(on_advise_trampoline),
                Some(on_disconnect_trampoline),
                Some(free_connection_callbacks),
            )
        };

        IPCConnection { ptr, owned: true }
    }
}

impl Default for IPCConnectionBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// =============================================================================
// IPCServer
// =============================================================================

/// Callback data for the server's OnAcceptConnection.
struct ServerCallbacks {
    on_accept: AcceptConnectionCallback,
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn on_accept_connection_trampoline(user_data: *mut c_void, topic: *const i8) -> *mut ffi::wxd_IPCConnection_t {
    if user_data.is_null() {
        return ptr::null_mut();
    }
    let callbacks = &mut *(user_data as *mut ServerCallbacks);
    let topic_str = if topic.is_null() {
        ""
    } else {
        CStr::from_ptr(topic).to_str().unwrap_or("")
    };
    if let Some(conn) = (callbacks.on_accept)(topic_str) {
        // Transfer ownership to C++ - it will manage the connection
        let ptr = conn.ptr;
        std::mem::forget(conn);
        ptr
    } else {
        ptr::null_mut()
    }
}

#[allow(unsafe_op_in_unsafe_fn)]
unsafe extern "C" fn free_server_callbacks(user_data: *mut c_void) {
    if !user_data.is_null() {
        let _ = Box::from_raw(user_data as *mut ServerCallbacks);
    }
}

/// An IPC server that listens for client connections.
///
/// The server listens on a service and accepts connections from clients.
/// On Windows, the service name identifies a DDE service. On Unix/macOS, the service
/// can be a port number (TCP) or a file path (Unix domain socket).
/// When a client connects, the OnAcceptConnection callback is called, which should
/// return a new IPCConnection to handle the client.
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::ipc::{IPCServer, IPCConnection, IPCFormat};
///
/// let server = IPCServer::new(|topic| {
///     println!("Client connecting to topic: {}", topic);
///     Some(IPCConnection::builder()
///         .on_execute(|_topic, data, _format| {
///             println!("Received: {:?}", String::from_utf8_lossy(data));
///             true
///         })
///         .build())
/// });
///
/// if server.create("4242") {
///     println!("Server listening on port 4242");
/// }
/// ```
pub struct IPCServer {
    ptr: *mut ffi::wxd_IPCServer_t,
}

impl IPCServer {
    /// Create a new IPC server with the given OnAcceptConnection callback.
    ///
    /// The callback receives the topic string and should return Some(IPCConnection)
    /// to accept the connection, or None to reject it.
    pub fn new<F>(on_accept_connection: F) -> Self
    where
        F: FnMut(&str) -> Option<IPCConnection> + 'static,
    {
        let callbacks = ServerCallbacks {
            on_accept: Box::new(on_accept_connection),
        };
        let user_data = Box::into_raw(Box::new(callbacks)) as *mut c_void;

        let ptr =
            unsafe { ffi::wxd_IPCServer_Create(user_data, Some(on_accept_connection_trampoline), Some(free_server_callbacks)) };

        Self { ptr }
    }

    /// Start the server listening on the given service.
    ///
    /// The service can be a port number (e.g., "4242") or a Unix socket path.
    /// Returns true if the server started successfully.
    pub fn create(&self, service: &str) -> bool {
        if self.ptr.is_null() {
            return false;
        }
        let c_service = match CString::new(service) {
            Ok(s) => s,
            Err(_) => return false,
        };
        unsafe { ffi::wxd_IPCServer_Create_Service(self.ptr, c_service.as_ptr()) }
    }
}

impl Drop for IPCServer {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_IPCServer_Destroy(self.ptr) };
        }
    }
}

// =============================================================================
// IPCClient
// =============================================================================

/// An IPC client that connects to servers.
///
/// The client connects to a server on a given host, service, and topic.
/// On Windows, DDE is used (host is ignored for local DDE connections). On Unix/macOS,
/// TCP sockets or Unix domain sockets are used depending on the service format.
/// If the connection is successful, it returns an IPCConnection that can be
/// used to exchange data with the server.
///
/// # Example
///
/// ```rust,no_run
/// use wxdragon::ipc::{IPCClient, IPCFormat};
///
/// let client = IPCClient::new();
///
/// // Connect to a server
/// if let Some(conn) = client.make_connection("localhost", "4242", "test") {
///     // Send a command
///     conn.execute_string("Hello, server!");
///
///     // Request data
///     if let Some(data) = conn.request("status", IPCFormat::Text) {
///         println!("Server status: {}", String::from_utf8_lossy(&data));
///     }
///
///     // Disconnect when done
///     conn.disconnect();
/// }
/// ```
pub struct IPCClient {
    ptr: *mut ffi::wxd_IPCClient_t,
}

impl IPCClient {
    /// Create a new IPC client.
    pub fn new() -> Self {
        let ptr = unsafe { ffi::wxd_IPCClient_Create() };
        Self { ptr }
    }

    /// Connect to a server.
    ///
    /// # Arguments
    ///
    /// * `host` - The hostname or IP address of the server
    /// * `service` - The service (port number or socket path)
    /// * `topic` - The topic to connect to
    ///
    /// Returns Some(IPCConnection) if the connection was successful, None otherwise.
    pub fn make_connection(&self, host: &str, service: &str, topic: &str) -> Option<IPCConnection> {
        self.make_connection_with_callbacks(host, service, topic, IPCConnectionBuilder::new())
    }

    /// Connect to a server with custom callbacks.
    ///
    /// This allows you to specify callbacks for the connection (e.g., OnAdvise
    /// for receiving server-pushed updates).
    pub fn make_connection_with_callbacks(
        &self,
        host: &str,
        service: &str,
        topic: &str,
        builder: IPCConnectionBuilder,
    ) -> Option<IPCConnection> {
        if self.ptr.is_null() {
            return None;
        }

        let c_host = CString::new(host).ok()?;
        let c_service = CString::new(service).ok()?;
        let c_topic = CString::new(topic).ok()?;

        let callbacks_box = Box::new(builder.callbacks);
        let user_data = Box::into_raw(callbacks_box) as *mut c_void;

        let conn_ptr = unsafe {
            ffi::wxd_IPCClient_MakeConnection(
                self.ptr,
                c_host.as_ptr(),
                c_service.as_ptr(),
                c_topic.as_ptr(),
                user_data,
                Some(on_execute_trampoline),
                Some(on_request_trampoline),
                Some(on_poke_trampoline),
                Some(on_start_advise_trampoline),
                Some(on_stop_advise_trampoline),
                Some(on_advise_trampoline),
                Some(on_disconnect_trampoline),
                Some(free_connection_callbacks),
            )
        };

        if conn_ptr.is_null() {
            None
        } else {
            Some(IPCConnection {
                ptr: conn_ptr,
                owned: false, // Owned by the wxWidgets system
            })
        }
    }
}

impl Default for IPCClient {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for IPCClient {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { ffi::wxd_IPCClient_Destroy(self.ptr) };
        }
    }
}
