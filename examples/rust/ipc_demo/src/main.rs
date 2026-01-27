//! IPC Demo - demonstrates inter-process communication using wxDragon
//!
//! This demo shows:
//! - Server: Listens for connections and receives messages
//! - Client: Connects to server and sends messages
//!
//! Run two separate instances:
//! - First instance: `cargo run -p ipc_demo` (server mode - click "Start Server")
//! - Second instance: `cargo run -p ipc_demo` (client mode - click "Connect as Client")

use std::cell::RefCell;
use std::rc::Rc;
use wxdragon::ipc::{IPCClient, IPCConnection, IPCConnectionBuilder, IPCFormat, IPCServer};
use wxdragon::prelude::*;

const SERVICE_PORT: &str = "4242";
const TOPIC: &str = "wxdragon_ipc_demo";

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    SystemOptions::set_option_by_int("msw.no-manifest-check", 1);

    let _ = wxdragon::main(|_| {
        create_ui();
    });
}

fn create_ui() {
    let frame = Frame::builder().with_title("IPC Demo").with_size(Size::new(500, 400)).build();

    let panel = Panel::builder(&frame).build();
    let main_sizer = BoxSizer::builder(Orientation::Vertical).build();

    // Status display
    let status_label = StaticText::builder(&panel).with_label("Status: Not connected").build();

    // Log area
    let log_text = TextCtrl::builder(&panel)
        .with_style(TextCtrlStyle::MultiLine | TextCtrlStyle::ReadOnly)
        .build();

    // Message input
    let input_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let input_text = TextCtrl::builder(&panel).build();
    let send_button = Button::builder(&panel).with_label("Send").build();
    input_sizer.add(&input_text, 1, SizerFlag::Expand | SizerFlag::All, 5);
    input_sizer.add(&send_button, 0, SizerFlag::All, 5);

    // Control buttons
    let button_sizer = BoxSizer::builder(Orientation::Horizontal).build();
    let server_button = Button::builder(&panel).with_label("Start Server").build();
    let client_button = Button::builder(&panel).with_label("Connect as Client").build();
    let disconnect_button = Button::builder(&panel).with_label("Disconnect").build();

    button_sizer.add(&server_button, 0, SizerFlag::All, 5);
    button_sizer.add(&client_button, 0, SizerFlag::All, 5);
    button_sizer.add(&disconnect_button, 0, SizerFlag::All, 5);

    // Layout
    main_sizer.add(&status_label, 0, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add(&log_text, 1, SizerFlag::Expand | SizerFlag::All, 5);
    main_sizer.add_sizer(&input_sizer, 0, SizerFlag::Expand, 0);
    main_sizer.add_sizer(&button_sizer, 0, SizerFlag::AlignCentre | SizerFlag::All, 5);

    panel.set_sizer(main_sizer, true);

    // Shared state - only track what mode we're in and client connection
    let is_server: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
    let server: Rc<RefCell<Option<IPCServer>>> = Rc::new(RefCell::new(None));
    let client: Rc<RefCell<Option<IPCClient>>> = Rc::new(RefCell::new(None));
    // Client connection - only valid for client mode
    let client_connection: Rc<RefCell<Option<IPCConnection>>> = Rc::new(RefCell::new(None));

    // Helper to append to log
    let log_text_clone = log_text;
    let append_log = Rc::new(move |msg: &str| {
        let current = log_text_clone.get_value();
        let new_text = if current.is_empty() {
            msg.to_string()
        } else {
            format!("{}\n{}", current, msg)
        };
        log_text_clone.set_value(&new_text);
    });

    // Start Server button
    {
        let server = server.clone();
        let is_server = is_server.clone();
        let status_label_server = status_label;
        let append_log = append_log.clone();

        server_button.on_click(move |_| {
            if server.borrow().is_some() {
                append_log("Server already running");
                return;
            }

            let append_log_clone = append_log.clone();

            // Create server - the callback creates connections for incoming clients
            let ipc_server = IPCServer::new(move |topic| {
                append_log_clone(&format!("Client connected to topic: {}", topic));

                // Create a connection for this client with callbacks to handle incoming data
                let append_log_exec = append_log_clone.clone();
                let conn = IPCConnection::builder()
                    .on_execute(move |_topic, data, _format| {
                        let msg = String::from_utf8_lossy(data);
                        append_log_exec(&format!("[Server] Received: {}", msg));
                        true
                    })
                    .on_disconnect({
                        let append_log_disc = append_log_clone.clone();
                        move || {
                            append_log_disc("[Server] Client disconnected");
                            true
                        }
                    })
                    .build();

                Some(conn)
            });

            if ipc_server.create(SERVICE_PORT) {
                append_log(&format!("Server started on port {}", SERVICE_PORT));
                append_log("Waiting for client connections...");
                append_log("(Run another instance and click 'Connect as Client')");
                status_label_server.set_label("Status: Server running");
                *server.borrow_mut() = Some(ipc_server);
                *is_server.borrow_mut() = true;
            } else {
                append_log("Failed to start server - port may be in use");
            }
        });
    }

    // Connect as Client button
    {
        let client = client.clone();
        let client_connection = client_connection.clone();
        let is_server = is_server.clone();
        let status_label_client = status_label;
        let append_log = append_log.clone();

        client_button.on_click(move |_| {
            if *is_server.borrow() {
                append_log("Already running as server");
                return;
            }

            if client_connection.borrow().is_some() {
                append_log("Already connected as client");
                return;
            }

            let ipc_client = IPCClient::new();
            let append_log_clone = append_log.clone();

            // Create connection with callbacks for client-side events
            let conn_builder = IPCConnectionBuilder::new()
                .on_advise(move |_topic, item, data, _format| {
                    let msg = String::from_utf8_lossy(data);
                    append_log_clone(&format!("[Client] Server advise {}: {}", item, msg));
                    true
                })
                .on_disconnect({
                    let append_log_disc = append_log.clone();
                    move || {
                        append_log_disc("[Client] Disconnected from server");
                        true
                    }
                });

            match ipc_client.make_connection_with_callbacks("localhost", SERVICE_PORT, TOPIC, conn_builder) {
                Some(conn) => {
                    append_log(&format!("Connected to server at localhost:{}", SERVICE_PORT));
                    status_label_client.set_label("Status: Connected as client");
                    *client_connection.borrow_mut() = Some(conn);
                    *client.borrow_mut() = Some(ipc_client);
                }
                None => {
                    append_log("Failed to connect - is the server running?");
                    append_log("Start another instance and click 'Start Server' first");
                }
            }
        });
    }

    // Disconnect button
    {
        let server = server.clone();
        let client = client.clone();
        let client_connection = client_connection.clone();
        let is_server = is_server.clone();
        let status_label_disconnect = status_label;
        let append_log = append_log.clone();

        disconnect_button.on_click(move |_| {
            // For client: disconnect and drop connection
            if let Some(conn) = client_connection.borrow_mut().take() {
                conn.disconnect();
                append_log("Disconnected from server");
            }
            *client.borrow_mut() = None;

            // For server: stop the server
            if server.borrow().is_some() {
                *server.borrow_mut() = None;
                append_log("Server stopped");
            }

            *is_server.borrow_mut() = false;
            status_label_disconnect.set_label("Status: Not connected");
        });
    }

    // Send button (client only)
    {
        let client_connection = client_connection.clone();
        let is_server = is_server.clone();
        let append_log = append_log.clone();

        send_button.on_click(move |_| {
            let msg = input_text.get_value();
            if msg.is_empty() {
                return;
            }

            if *is_server.borrow() {
                append_log("Server mode - clients send data TO server, not the other way");
                append_log("(Use Advise for server->client push notifications)");
                return;
            }

            if let Some(ref conn) = *client_connection.borrow() {
                if conn.execute(msg.as_bytes(), IPCFormat::Utf8Text) {
                    append_log(&format!("[Client] Sent: {}", msg));
                    input_text.set_value("");
                } else {
                    append_log("Failed to send message");
                }
            } else {
                append_log("Not connected - click 'Connect as Client' first");
            }
        });
    }

    // Initial instructions
    {
        let append_log = append_log.clone();
        append_log("=== wxDragon IPC Demo ===");
        append_log("To test IPC, run TWO instances of this program:");
        append_log("1. In first instance: Click 'Start Server'");
        append_log("2. In second instance: Click 'Connect as Client'");
        append_log("3. Type messages in client and click 'Send'");
        append_log("");
    }

    frame.show(true);
    frame.centre();
}
