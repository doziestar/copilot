use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::sleep;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use base64;
use reqwest;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use rodio::{Decoder, OutputStream, Sink};

const LISTEN_DURATION: u64 = 10;
const AMPLIFICATION_FACTOR: f32 = 10.0;
const NOISE_GATE_THRESHOLD: f32 = 0.005;

#[derive(Serialize)]
struct SpeechRecognizeRequest {
    config: RecognitionConfig,
    audio: RecognitionAudio,
}

#[derive(Serialize)]
struct RecognitionConfig {
    encoding: String,
    sample_rate_hertz: u32,
    language_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    audio_channel_count: Option<i32>,
}

#[derive(Serialize)]
struct RecognitionAudio {
    content: String,
}

#[derive(Deserialize, Debug)]
struct SpeechRecognizeResponse {
    results: Vec<SpeechRecognitionResult>,
}

#[derive(Deserialize, Debug)]
struct SpeechRecognitionResult {
    alternatives: Vec<SpeechRecognitionAlternative>,
    #[serde(default)]
    is_final: bool,
}

#[derive(Deserialize, Debug)]
struct SpeechRecognitionAlternative {
    transcript: String,
    confidence: Option<f32>,
}

fn find_closest_supported_sample_rate(
    supported_configs: cpal::SupportedInputConfigs,
    target_rate: u32,
) -> Option<cpal::SupportedStreamConfig> {
    supported_configs
        .map(|config| {
            let rate = if target_rate < config.min_sample_rate().0 {
                config.min_sample_rate()
            } else if target_rate > config.max_sample_rate().0 {
                config.max_sample_rate()
            } else {
                cpal::SampleRate(target_rate)
            };
            (config.with_sample_rate(rate), (target_rate as i32 - rate.0 as i32).abs())
        })
        .min_by_key(|&(_, diff)| diff)
        .map(|(config, _)| config)
}

pub async fn listen_for_speech() -> Result<String, Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_input_device().ok_or("No input device available")?;

    let config = device.default_input_config()?;
    println!("Default input config: {:?}", config);

    let sample_rate = config.sample_rate().0;
    let channels = config.channels();

    let audio_data = Arc::new(Mutex::new(Vec::new()));
    let audio_data_clone = Arc::clone(&audio_data);

    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut audio_buffer = audio_data_clone.blocking_lock();
            audio_buffer.extend_from_slice(data);
        },
        |err| eprintln!("An error occurred on stream: {}", err),
        None,
    )?;

    stream.play()?;

    println!("Listening for speech... ({} seconds)", LISTEN_DURATION);

    for i in 1..=LISTEN_DURATION {
        sleep(Duration::from_secs(1)).await;
        let len = audio_data.lock().await.len();
        let max_amplitude = audio_data.lock().await.iter().map(|&x| x.abs()).fold(0.0, f32::max);
        println!("Second {}: Received {} samples, Max amplitude: {}", i, len, max_amplitude);
    }

    stream.pause()?;

    let audio_vec = audio_data.lock().await.clone();
    println!("Recognizing speech... {} samples collected", audio_vec.len());

    let amplified: Vec<i16> = audio_vec.into_iter()
        .map(|sample| {
            let amplified = sample * AMPLIFICATION_FACTOR;
            if amplified.abs() > NOISE_GATE_THRESHOLD {
                (amplified * i16::MAX as f32) as i16
            } else {
                0
            }
        })
        .collect();

    let audio_bytes: Vec<u8> = amplified.iter().flat_map(|&sample| sample.to_le_bytes()).collect();
    let audio_content = base64::encode(&audio_bytes);

    let client = reqwest::Client::new();
    let api_key = std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set");

    let request = SpeechRecognizeRequest {
        config: RecognitionConfig {
            encoding: "LINEAR16".to_string(),
            sample_rate_hertz: sample_rate,
            language_code: "en-US".to_string(),
            audio_channel_count: Some(channels as i32),
        },
        audio: RecognitionAudio {
            content: audio_content,
        },
    };

    let response = client
        .post("https://speech.googleapis.com/v1/speech:recognize")
        .query(&[("key", api_key)])
        .json(&request)
        .send()
        .await?;

    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);

    let response_json: SpeechRecognizeResponse = serde_json::from_str(&response_text)?;

    response_json.results
        .first()
        .and_then(|result| result.alternatives.first())
        .map(|alt| alt.transcript.clone())
        .ok_or_else(|| "No speech recognized".into())
}

pub fn speak(text: &str) -> Result<(), Box<dyn Error>> {
    let (_stream, stream_handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&stream_handle)?;

    // This is a placeholder. In a real implementation, you'd use a TTS engine to generate audio
    let owned_text = text.to_owned();
    let file = std::io::Cursor::new(owned_text);
    let source = Decoder::new(file)?;
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}