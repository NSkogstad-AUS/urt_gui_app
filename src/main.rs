mod utils;

use eframe::{egui, App, NativeOptions};
use std::thread;

#[derive(Default)]
struct AppState {
    label_text: String,
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show the label
            ui.label(&self.label_text);

            // Add some spacing for visual separation
            ui.add_space(20.0);

            // Add a button in the middle to play audio
            if ui.button("Play Audio").clicked() {
                // Call the GStreamer function from utils.rs
                match utils::play_audio() {
                    Ok(message) => self.label_text = message,
                    Err(error) => self.label_text = error,
                }
            }

            // Add a button to simulate stream notifications
            if ui.button("Start UDP Stream Notifications").clicked() {
                // Placeholder for starting a UDP socket listener
                thread::spawn(|| {
                    if let Err(e) = utils::start_udp_listener() {
                        eprintln!("Failed to start UDP listener: {}", e);
                    }
                });
            }
        });
    }
}

fn main() {
    let app = AppState {
        label_text: "Hello, world!".into(),
    };
    
    let native_options = NativeOptions::default();
    eframe::run_native(
        "Hello Egui",
        native_options,
        Box::new(|_cc| Box::new(app)),
    );
}


// echo "Test Message" | nc -u 127.0.0.1 34254
// Input the above into a terminal window to test it out