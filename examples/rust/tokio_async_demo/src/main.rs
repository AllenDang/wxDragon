//! Tokio Async Integration Demo
//!
//! This example demonstrates how to integrate tokio async runtime with wxDragon
//! using the idle event mechanism. It shows the recommended pattern for handling
//! async messages in a GUI application.
//!
//! This example uses idle events with smart RequestMore() control for optimal
//! performance - idle events are only requested when there are actually messages
//! to process, avoiding unnecessary CPU usage.

use std::time::Duration;
use tokio::sync::mpsc as tokio_mpsc;
use wxdragon::prelude::*;

/// Message types that can be sent from async tasks to the GUI
#[derive(Debug, Clone)]
enum AsyncMessage {
    UpdateCounter(i32),
    UpdateStatus(String),
    TaskCompleted(String),
}

/// A simple async message handler that processes tokio messages in the GUI thread
struct AsyncMessageHandler {
    receiver: tokio_mpsc::UnboundedReceiver<AsyncMessage>,
    counter_text: StaticText,
    status_text: StaticText,
}

impl AsyncMessageHandler {
    fn new(receiver: tokio_mpsc::UnboundedReceiver<AsyncMessage>, counter_text: StaticText, status_text: StaticText) -> Self {
        Self {
            receiver,
            counter_text,
            status_text,
        }
    }

    /// Process pending async messages
    /// Returns true if there are more messages to process
    fn process_messages(&mut self) -> bool {
        let mut processed_count = 0;

        // Process up to 10 messages per idle event to avoid blocking the GUI
        for _ in 0..10 {
            match self.receiver.try_recv() {
                Ok(message) => {
                    processed_count += 1;
                    println!("Processing message: {message:?}");
                    self.handle_message(message);
                }
                Err(tokio_mpsc::error::TryRecvError::Empty) => {
                    // No more messages
                    break;
                }
                Err(tokio_mpsc::error::TryRecvError::Disconnected) => {
                    // Channel disconnected, no more messages will come
                    println!("Channel disconnected");
                    return false;
                }
            }
        }

        let has_more = processed_count == 10;
        if processed_count > 0 {
            println!("Processed {processed_count} messages, has_more: {has_more}");
        }

        // If we processed the maximum number of messages, there might be more
        // If we processed fewer, we've emptied the queue
        has_more
    }

    fn handle_message(&mut self, message: AsyncMessage) {
        match message {
            AsyncMessage::UpdateCounter(count) => {
                self.counter_text.set_label(&format!("Counter: {count}"));
            }
            AsyncMessage::UpdateStatus(status) => {
                self.status_text.set_label(&status);
            }
            AsyncMessage::TaskCompleted(task_name) => {
                self.status_text.set_label(&format!("Task completed: {task_name}"));
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Set idle mode to only send events to windows that request them
    // This is more efficient when we have async work to do
    IdleEvent::set_mode(IdleMode::ProcessSpecified);

    let _ = wxdragon::main(|_| {
        // Create the main frame
        let frame = Frame::builder()
            .with_title("Tokio Async Integration Demo")
            .with_position(Point::new(100, 100))
            .with_size(Size::new(400, 300))
            .build();

        // Create UI elements
        let counter_text = StaticText::builder(&frame)
            .with_label("Counter: 0")
            .with_pos(Point::new(20, 20))
            .with_size(Size::new(200, 30))
            .build();

        let status_text = StaticText::builder(&frame)
            .with_label("Status: Ready")
            .with_pos(Point::new(20, 60))
            .with_size(Size::new(350, 30))
            .build();

        let start_button = Button::builder(&frame)
            .with_label("Start Async Tasks")
            .with_pos(Point::new(20, 100))
            .with_size(Size::new(150, 30))
            .build();

        let stop_button = Button::builder(&frame)
            .with_label("Stop Tasks")
            .with_pos(Point::new(180, 100))
            .with_size(Size::new(100, 30))
            .build();

        // Create tokio channel for async communication
        let (sender, receiver) = tokio_mpsc::unbounded_channel::<AsyncMessage>();

        // Create our async message handler
        let mut message_handler = AsyncMessageHandler::new(receiver, counter_text, status_text);

        // Use idle events for efficient async message processing
        // This is the recommended approach for wxDragon async integration
        frame.on_idle(move |event_data| {
            let has_more_messages = message_handler.process_messages();

            if let WindowEventData::Idle(event) = event_data {
                // Only request more idle events if we have messages to process
                // This reduces CPU usage when there's no async work to do
                if has_more_messages {
                    println!("Requesting more idle events (has messages)");
                    event.request_more(true);
                } else {
                    // No more messages, stop requesting idle events to save CPU
                    event.request_more(false);
                }
            }
        });

        // Handle start button click
        let sender_clone = sender.clone();
        start_button.on_click(move |_| {
            println!("Start button clicked!");
            let sender = sender_clone.clone();

            // Spawn async tasks
            tokio::spawn(async move {
                println!("Spawning async tasks...");

                // Counter task - updates every 500ms
                let sender_counter = sender.clone();
                tokio::spawn(async move {
                    println!("Counter task started");
                    for i in 1..=20 {
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        println!("Sending counter update: {i}");
                        if sender_counter.send(AsyncMessage::UpdateCounter(i)).is_err() {
                            println!("Failed to send counter message");
                            break;
                        }
                    }
                    let _ = sender_counter.send(AsyncMessage::TaskCompleted("Counter".to_string()));
                });

                // Status task - updates every 1 second
                let sender_status = sender.clone();
                tokio::spawn(async move {
                    println!("Status task started");
                    let statuses = [
                        "Processing...",
                        "Fetching data...",
                        "Computing results...",
                        "Almost done...",
                        "Finalizing...",
                    ];

                    for status in statuses.iter() {
                        tokio::time::sleep(Duration::from_secs(1)).await;
                        println!("Sending status update: {status}");
                        if sender_status.send(AsyncMessage::UpdateStatus(status.to_string())).is_err() {
                            println!("Failed to send status message");
                            break;
                        }
                    }
                    let _ = sender_status.send(AsyncMessage::TaskCompleted("Status Updates".to_string()));
                });

                // Simulate some async work
                let sender_work = sender.clone();
                tokio::spawn(async move {
                    println!("Background work task started");
                    tokio::time::sleep(Duration::from_secs(3)).await;
                    let _ = sender_work.send(AsyncMessage::UpdateStatus("Background work completed!".to_string()));
                    let _ = sender_work.send(AsyncMessage::TaskCompleted("Background Work".to_string()));
                });
            });
        });

        // Handle stop button click
        stop_button.on_click(move |_| {
            // In a real application, you would send a cancellation signal to your async tasks
            // For this demo, we just update the status
            status_text.set_label("Tasks stopped (demo - tasks will continue)");
        });

        // In ProcessSpecified mode, we need to manually request the first idle event
        // to start the cycle. We'll do this by using the low-level event binding
        // to trigger an initial idle event after the frame is shown.

        // First, show the frame so it's ready to receive events
        frame.show(true);
        frame.centre();

        // Enable idle event processing for this frame
        frame.set_extra_style(ExtraWindowStyle::ProcessIdle);
    });
}
