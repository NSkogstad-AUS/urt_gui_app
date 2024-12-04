use gstreamer::prelude::*;
use gstreamer::{ElementFactory, Pipeline, State, MessageView};
use std::net::UdpSocket;

pub fn play_video() -> Result<String, String> {
    // Initialize GStreamer
    println!("Initializing GStreamer...");
    gstreamer::init().map_err(|e| format!("Failed to initialize GStreamer: {:?}", e))?;

    // Create a new pipeline
    println!("Creating pipeline...");
    let pipeline = Pipeline::new(None);

    // Use avfvideosrc for macOS camera input
    println!("Creating avfvideosrc element...");
    let source = ElementFactory::make("avfvideosrc")
        .build()
        .map_err(|e| format!("Failed to create avfvideosrc element: {:?}", e))?;

    // Convert the video to a format suitable for rendering
    println!("Creating videoconvert element...");
    let converter = ElementFactory::make("videoconvert")
        .build()
        .map_err(|e| format!("Failed to create videoconvert element: {:?}", e))?;

    // Use glimagesink to render video
    println!("Creating glimagesink element...");
    let sink = ElementFactory::make("glimagesink")
        .build()
        .map_err(|e| format!("Failed to create glimagesink element: {:?}", e))?;

    // Add elements to the pipeline
    println!("Adding elements to the pipeline...");
    pipeline
        .add_many(&[&source, &converter, &sink])
        .map_err(|_| "Failed to add elements to the pipeline".to_string())?;

    // Link the elements together
    println!("Linking elements...");
    source.link(&converter).map_err(|_| "Failed to link source and converter".to_string())?;
    converter.link(&sink).map_err(|_| "Failed to link converter and sink".to_string())?;

    // Start playing the video
    println!("Setting pipeline state to Playing...");
    pipeline
        .set_state(State::Playing)
        .map_err(|_| "Failed to set pipeline state to playing".to_string())?;

    println!("Pipeline is playing.");

    // Listen for messages on the bus
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of stream");
                break;
            }
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {:?}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            _ => (),
        }
    }

    Ok("Playing video from camera...".into())
}

// Function to start a UDP listener for stream notifications
pub fn start_udp_listener<F>(callback: F) -> Result<(), String>
where
    F: Fn(String) + Send + 'static,
{
    let socket = UdpSocket::bind("0.0.0.0:34254").map_err(|e| format!("Failed to bind UDP socket: {}", e))?;
    println!("Listening for stream notifications on UDP port 34254...");
    
    let mut buf = [0; 1024];
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                let message = String::from_utf8_lossy(&buf[..amt]).to_string();
                println!("Received {} bytes from {}: {:?}", amt, src, message);
                callback(message);
            }
            Err(e) => {
                eprintln!("UDP receive error: {}", e);
                break;
            }
        }
    }
    Ok(())
}

pub fn play_video_stream() -> Result<String, String> {
    // Initialize GStreamer
    println!("Initializing GStreamer...");
    gstreamer::init().map_err(|e| format!("Failed to initialize GStreamer: {:?}", e))?;

    // Create a new pipeline
    println!("Creating pipeline...");
    let pipeline = Pipeline::new(None);

    // Use udpsrc to receive video stream
    println!("Creating udpsrc element...");
    let source = ElementFactory::make("udpsrc")
        .property("port", 5000)
        .build()
        .map_err(|e| format!("Failed to create udpsrc element: {:?}", e))?;

    // Set caps for the udpsrc element
    println!("Setting caps for udpsrc element...");
    let caps = gstreamer::Caps::builder("application/x-rtp")
        .field("media", &"video")
        .field("encoding-name", &"H264")
        .build();

    source.set_property("caps", &caps);

    // Use application/x-rtp caps
    println!("Creating rtph264depay element...");
    let depay = ElementFactory::make("rtph264depay")
        .build()
        .map_err(|e| format!("Failed to create rtph264depay element: {:?}", e))?;

    // Decode the video stream
    println!("Creating avdec_h264 element...");
    let decoder = ElementFactory::make("avdec_h264")
        .build()
        .map_err(|e| format!("Failed to create avdec_h264 element: {:?}", e))?;

    // Convert the video to a format suitable for rendering
    println!("Creating videoconvert element...");
    let converter = ElementFactory::make("videoconvert")
        .build()
        .map_err(|e| format!("Failed to create videoconvert element: {:?}", e))?;

    // Use glimagesink to render video
    println!("Creating glimagesink element...");
    let sink = ElementFactory::make("glimagesink")
        .build()
        .map_err(|e| format!("Failed to create glimagesink element: {:?}", e))?;

    // Add elements to the pipeline
    println!("Adding elements to the pipeline...");
    pipeline
        .add_many(&[&source, &depay, &decoder, &converter, &sink])
        .map_err(|_| "Failed to add elements to the pipeline".to_string())?;

    // Link the elements together
    println!("Linking elements...");
    source.link(&depay).map_err(|_| "Failed to link source and depay".to_string())?;
    depay.link(&decoder).map_err(|_| "Failed to link depay and decoder".to_string())?;
    decoder.link(&converter).map_err(|_| "Failed to link decoder and converter".to_string())?;
    converter.link(&sink).map_err(|_| "Failed to link converter and sink".to_string())?;

    // Start playing the video stream
    println!("Setting pipeline state to Playing...");
    pipeline
        .set_state(State::Playing)
        .map_err(|_| "Failed to set pipeline state to playing".to_string())?;

    println!("Pipeline is playing.");

    // Listen for messages on the bus
    let bus = pipeline.bus().unwrap();
    for msg in bus.iter_timed(gstreamer::ClockTime::NONE) {
        match msg.view() {
            MessageView::Eos(..) => {
                println!("End of stream");
                break;
            }
            MessageView::Error(err) => {
                eprintln!(
                    "Error received from element {:?}: {:?}",
                    err.src().map(|s| s.path_string()),
                    err.error()
                );
                eprintln!("Debugging information: {:?}", err.debug());
                break;
            }
            _ => (),
        }
    }

    Ok("Playing video stream...".into())
}
