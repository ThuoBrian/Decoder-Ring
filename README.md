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

## 🛠️ Usage

### Clone and Run
```bash
git clone https://github.com/your-username/caesar-decryptor.git
cd caesar-decryptor
cargo run
