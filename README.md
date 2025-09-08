# Caesar Cipher Decryptor (with Frequency Analysis)

![Rust](https://img.shields.io/badge/language-Rust-orange?logo=rust&logoColor=white)
![Cryptography](https://img.shields.io/badge/topic-Cryptography-blue)
![Status](https://img.shields.io/badge/status-Work_in_Progress-yellow)
![License](https://img.shields.io/badge/license-MIT-green)

A Rust-based Caesar cipher decryption tool that uses **frequency analysis** to crack encrypted text.  

---

## ✨ Features
- 🔡 **Letter Frequency Analysis**: Calculates character distribution of input text.  
- 📊 **English Frequency Comparison**: Matches text frequency against standard English letter frequencies.  
- 🔐 **Decryption**: Supports Caesar cipher decryption with a known shift.  
- 🤖 **Shift Guessing**: Automatically detects the most likely shift using scoring.  
- ⚡ **Scoring System**: Evaluates multiple shifts and selects the best candidate.  

---

## 🖥️ Command Line Usage

This project uses [`clap`](https://docs.rs/clap) to provide a CLI interface.

### Run with Cargo
```bash
cargo run -- --message "Uifsf jt b tfdsfu dpef!" --stats
cargo run -- --message "Uifsf jt b tfdsfu dpef!" --guess

-m, --message <MESSAGE>   The message to decrypt
-s, --stats               Show statistical frequency analysis of the message
-g, --guess               Attempt to guess the Caesar cipher shift automatically

# Show stats for encrypted text
cargo run -- --message "Uifsf jt b tfdsfu dpef!" --stats

# Guess the shift and decrypt automatically
cargo run -- --message "Uifsf jt b tfdsfu dpef!" --guess
