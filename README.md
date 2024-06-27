# Copilot

## Overview

Copilot is an advanced assistant that uses speech recognition, natural language processing, and screen analysis to provide real-time assistance for various tasks. It combines multiple technologies to create an interactive and responsive AI companion.

## Features

- **Speech Recognition**: Captures and transcribes spoken input in real-time.
- **Natural Language Processing**: Utilizes OpenAI's API to generate contextual responses.
- **Text-to-Speech**: Converts AI responses to audible speech.
- **Screen Capture and OCR**: Analyzes on-screen content to provide context-aware assistance.
- **Mouse Tracking**: Monitors cursor position for potential context cues.

## Prerequisites

- Rust (latest stable version)
- Cargo (Rust's package manager)
- A valid OpenAI API key
- A valid Google Cloud API key with Speech-to-Text API enabled
- Tesseract OCR installed on your system

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/doziestar/copilot.git
   cd copilot
   ```

2. Set up environment variables:
   ```
   export OPENAI_API_KEY=openai_api_key
   export GOOGLE_API_KEY=google_api_key
   ```

3. Install dependencies:
   ```
   cargo build
   ```

4. Install Tesseract OCR:
    - On macOS: `brew install tesseract`
    - On Ubuntu: `sudo apt-get install tesseract-ocr`
    - On Windows: Download and install from [Tesseract GitHub](https://github.com/UB-Mannheim/tesseract/wiki)

## Usage

Run the program with:

```
cargo run
```

Once started, the copilot will:
1. Listen for speech input for 10 seconds.
2. Transcribe and process the speech.
3. Generate an AI response.
4. Speak the response aloud.
5. Capture and analyze the screen content.
6. Track mouse position.

This cycle repeats continuously until the program is terminated.

## Configuration

You can modify the following parameters in the code:
- Speech recognition duration (default: 10 seconds)
- OpenAI model (default: "gpt-3.5-turbo")
- OCR language (default: English)

## Troubleshooting

- **No audio input**: Ensure your microphone is properly connected and set as the default input device.
- **Speech recognition errors**: Speak clearly and minimize background noise.
- **OCR not working**: Make sure Tesseract is properly installed and its path is correctly set.
- **API errors**: Verify that your API keys are correct and have the necessary permissions.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- OpenAI for their powerful language model API
- Google Cloud for their Speech-to-Text API
- The Rust community for excellent libraries and tools