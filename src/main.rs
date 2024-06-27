mod listen_for_speech;
mod capture_and_analyse_screen;
mod ai;

use std::error::Error;
use std::env;
use tokio::time::Duration;
use openai_api_rs::v1::api::Client;
use device_query::{DeviceQuery, DeviceState};
use crate::ai::generate_response;
use crate::capture_and_analyse_screen::capture_and_analyze_screen;
use crate::listen_for_speech::{listen_for_speech, speak};

const ITERATION_DELAY: u64 = 1; // seconds

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let openai_client = Client::new(env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set"));
    let device_state = DeviceState::new();

    loop {
        println!("Starting new iteration...");

        if let Err(e) = process_voice_input(&openai_client).await {
            eprintln!("Error in voice input processing: {}", e);
        }

        if let Err(e) = process_screen_capture(&openai_client).await {
            eprintln!("Error in screen capture processing: {}", e);
        }

        track_mouse_position(&device_state);

        println!("Iteration complete. Sleeping for {} second(s)...", ITERATION_DELAY);
        tokio::time::sleep(Duration::from_secs(ITERATION_DELAY)).await;
    }
}

async fn process_voice_input(openai_client: &Client) -> Result<(), Box<dyn Error>> {
    match listen_for_speech().await {
        Ok(text) if !text.is_empty() => {
            println!("Speech recognition result: {}", text);
            match generate_response(openai_client, &text) {
                Ok(response) => {
                    println!("AI response: {}", response);
                    speak(&response)?;
                }
                Err(e) => eprintln!("Response generation error: {}", e),
            }
        }
        Ok(_) => println!("No speech recognized"),
        Err(e) => eprintln!("Speech recognition error: {}", e),
    }
    Ok(())
}

async fn process_screen_capture(openai_client: &Client) -> Result<(), Box<dyn Error>> {
    match capture_and_analyze_screen() {
        Ok(screen_text) if !screen_text.is_empty() => {
            println!("Screen text captured: {}", screen_text);
            match generate_response(openai_client, &format!("Analyze this text from a screen capture: {}", screen_text)) {
                Ok(analysis) => println!("Screen analysis: {}", analysis),
                Err(e) => eprintln!("Screen analysis error: {}", e),
            }
        }
        Ok(_) => println!("No text found in screen capture"),
        Err(e) => eprintln!("Screen capture error: {}", e),
    }
    Ok(())
}

fn track_mouse_position(device_state: &DeviceState) {
    let mouse_pos = device_state.get_mouse().coords;
    println!("Mouse position: {:?}", mouse_pos);
}