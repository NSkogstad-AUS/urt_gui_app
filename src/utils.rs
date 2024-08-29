use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, State};
use gstreamer::glib;
use std::net::UdpSocket;

pub fn play_audio() -> Result<String, String> {
    // Initialize GStreamer
    gstreamer::init().map_err(|e| format!("Failed to initialize GStreamer: {:?}", e))?;

    // Create a new pipeline for local playback
    let pipeline = Pipeline::new(None);

    // Create elements for the pipeline using `.build()` to handle errors
    let source = ElementFactory::make("filesrc")
        .build()
        .map_err(|e| format!("Failed to create filesrc element: {:?}", e))?;
    let decoder = ElementFactory::make("decodebin")
        .build()
        .map_err(|e| format!("Failed to create decodebin element: {:?}", e))?;
    let sink = ElementFactory::make("autoaudiosink")
        .build()
        .map_err(|e| format!("Failed to create autoaudiosink element: {:?}", e))?;

    // Add elements to the pipeline
    pipeline
        .add_many(&[&source, &decoder, &sink])
        .map_err(|_| "Failed to add elements to the pipeline".to_string())?;

    // Link the elements together
    source.link(&decoder).map_err(|_| "Failed to link source and decoder".to_string())?;
    decoder.link(&sink).map_err(|_| "Failed to link decoder and sink".to_string())?;

    // Set the file to play (replace with your file path)
    source
        .set_property_from_str("location", "/Users/nicolaiskogstad/my_gui_app/Track 02_3.mp3");

    // Start playing
    pipeline
        .set_state(State::Playing)
        .map_err(|_| "Failed to set pipeline state to playing".to_string())?;

    Ok("Playing audio...".into())
}

pub fn start_udp_listener() -> Result<(), String> {
    let socket = UdpSocket::bind("127.0.0.1:34254").map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
    println!("Listening for stream notifications on UDP port 34254...");
    
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                println!("Received {} bytes from {}: {:?}", amt, src, &buf[..amt]);
                // Process the notification (e.g., start/stop streams)
            }
            Err(e) => {
                eprintln!("UDP receive error: {}", e);
                break;
            }
        }
    }
    Ok(())
}
