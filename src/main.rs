mod utils;

use eframe::{egui, App, NativeOptions};
use std::sync::{Arc, Mutex};
use std::thread;
use std::sync::mpsc::{self, Receiver};

#[derive(Default)]
struct AppState {
    label_text: Arc<Mutex<String>>,  // Wrap label_text in Arc<Mutex<>>
    receiver: Option<Receiver<String>>,  // Add a receiver to get messages from the UDP listener
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for new messages from the UDP listener
        if let Some(receiver) = &self.receiver {
            if let Ok(message) = receiver.try_recv() {
                println!("Received message: {}", message);  // Debug print
                let mut label_text = self.label_text.lock().unwrap();
                *label_text = message;
                ctx.request_repaint();
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            // Show the label (lock the mutex to access the label_text)
            let label_text = self.label_text.lock().unwrap();
            ui.label(&*label_text);

            // Add some spacing for visual separation
            ui.add_space(20.0);

            // Add a button to play video from the Mac's camera
            if ui.button("Play Video").clicked() {
                println!("Play Video button clicked.");
                // Clone the Arc to share it with the thread
                let label_text_clone = Arc::clone(&self.label_text);
                
                // Spawn a new thread for video playback to avoid freezing the UI
                thread::spawn(move || {
                    match utils::play_video() {
                        Ok(message) => {
                            let mut label = label_text_clone.lock().unwrap();
                            *label = message;
                        }
                        Err(error) => {
                            let mut label = label_text_clone.lock().unwrap();
                            *label = error;
                        }
                    }
                });
            }

            // Add a button to play video stream
            if ui.button("Play Video Stream").clicked() {
                println!("Play Video Stream button clicked.");
                // Clone the Arc to share it with the thread
                let label_text_clone = Arc::clone(&self.label_text);
                
                // Spawn a new thread for video stream playback to avoid freezing the UI
                thread::spawn(move || {
                    match utils::play_video_stream() { 
                        Ok(message) => {
                            let mut label = label_text_clone.lock().unwrap();
                            *label = message;
                        }
                        Err(error) => {
                            let mut label = label_text_clone.lock().unwrap();
                            *label = error;
                        }
                    }
                });
            }

            // Add a button to start UDP stream notifications
            if ui.button("Start UDP Stream Notifications").clicked() {
                // Create a channel to communicate with the UDP listener thread
                let (sender, receiver) = mpsc::channel();
                self.receiver = Some(receiver);

                // Spawn a new thread to run the UDP listener
                thread::spawn(move || {
                    if let Err(e) = utils::start_udp_listener(move |message| {
                        println!("Sending message: {}", message);  // Debug print
                        sender.send(message).unwrap();
                    }) {
                        eprintln!("Failed to start UDP listener: {}", e);
                    }
                });
            }
        });
    }
}

fn main() {
    let app = AppState {
        label_text: Arc::new(Mutex::new("Hello, Egui!".into())),
        receiver: None,
    };

    let native_options = NativeOptions::default();
    eframe::run_native(
        "Video Player with Egui",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}

// echo "Test Message" | nc -u 127.0.0.1 34254