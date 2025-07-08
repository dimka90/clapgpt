 ClapGPT

ClapGPT is a command-line interface (CLI) application that brings the power of OpenAI’s GPT models to your terminal. Built using the `clap` crate in Rust, it enables developers, students, and tech enthusiasts to interact with AI directly from the shell—without needing a GUI or web interface.

## 🚀 Features

### ✅ Currently Working
- 🔐 **Secure OpenAI Integration**: Communicates with OpenAI’s API using your API key stored securely in a `.env` file.
- 💬 **Chat Command**: Send messages and receive AI-generated responses from the command line.
- 🛠️ **Well-structured CLI**: Built with `clap` for intuitive commands and arguments.
- 📦 **JSON Response Parsing**: Neatly formats and extracts responses from OpenAI's structured response body.
- ⚙️ **Environment Configuration**: Uses `dotenv` to manage environment variables seamlessly.

### 🧪 Upcoming Features
- ⏳ **Conversation History**: Track session history and maintain context between prompts.
- 📋 **Clipboard Integration**: Copy response output to your system clipboard for quick access.
- 💾 **Log Output to File**: Save interactions to a log file for future reference or debugging.
- 🧠 **Custom GPT Model Selection**: Switch between GPT-3.5, GPT-4, or any other supported model.
- 🌐 **Proxy Support**: Enable usage in environments with restricted internet access.
- 🎛️ **Interactive Mode**: Run in a persistent session like a chatbot shell.

## 📦 Installation

```bash
git clone https://github.com/dimka90/clapgpt.git
cd clapgpt
cargo build --release
```
## 🔐 Setup Environment
Create a .env file in the clapgpt/ directory:
```
OPENAI_API_KEY=your_openai_api_key
```
## 🧑‍💻 Usage
```
cargo run -- clapgpt "Write a tweet about Rust and AI"
```

## 🔧 Commands
chat <prompt> – Send a prompt to OpenAI and receive a response.

More commands coming soon...

## 🤝 Contributing
Pull requests are welcome! For major changes, please open an issue first to discuss what you would like to change.

Fork the repo

Create a feature branch

Submit a PR
## 📄 License
This project is licensed under the MIT License – see the LICENSE file for details.
