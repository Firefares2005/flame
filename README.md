# 🔥 Flame

> A blazing fast HTTP client TUI (Terminal User Interface) written in Rust.

Flame is a lightweight, keyboard-driven alternative to Postman and Insomnia — it runs entirely in your terminal with no GUI overhead. Send HTTP requests, inspect responses with syntax highlighting, and save your collections locally.

![Rust](https://img.shields.io/badge/rust-1.75%2B-orange?logo=rust)
![License](https://img.shields.io/badge/license-MIT-blue)
![Platform](https://img.shields.io/badge/platform-Windows%20%7C%20macOS%20%7C%20Linux-lightgrey)

---

## ✨ Features

- 🚀 **7 HTTP methods** — GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS
- 🎨 **Colorful TUI** — built with `ratatui` + `crossterm`
- 📦 **JSON formatting** — pretty-print and syntax-highlight JSON responses
- ⏱️ **Response time** — measured in milliseconds
- 💾 **Collections** — save/load requests locally in TOML format
- 📜 **History** — track your last 50 requests
- 🔧 **Variables** — use `{{base_url}}` placeholders in any field
- 🗑️ **Delete collections** — remove saved requests with one key
- 📜 **Response scrolling** — navigate large responses easily
- ⚡ **Async** — powered by `tokio` + `reqwest`

---

## 📥 Installation

### Windows (one-line install)

Open **PowerShell** and run:

```powershell
irm https://raw.githubusercontent.com/Firefares2005/flame/main/install.ps1 | iex
```

Then restart PowerShell and run:

```powershell
flame
```

### Linux / macOS (one-line install)

Open a terminal and run:

```bash
curl -fsSL https://raw.githubusercontent.com/Firefares2005/flame/main/install.sh | bash
```

Then reload your shell and run:

```bash
flame
```

### Manual download

Grab the binary from the [latest release](https://github.com/Firefares2005/flame/releases/latest) and run it directly — no Rust needed.

### From source (for developers)

```bash
git clone https://github.com/Firefares2005/flame.git
cd flame
cargo run --release
```

---

## 📸 Demo

```
┌──────────────────────────────────────────────────────────┐
│ 🔥 flame                                                  │
│ 1 Request │ 2 Response │ 3 Collections                    │
├──────────────────────────────────────────────────────────┤
│ ┌────────┐ ┌──────────────────────────────────────────┐  │
│ │ GET    │ │ https://httpbin.org/get                  │  │
│ └────────┘ └──────────────────────────────────────────┘  │
│                                                            │
│ Headers (key: value per line)                             │
│ Accept: application/json                                  │
│                                                            │
│ Body                                                       │
│                                                            │
├──────────────────────────────────────────────────────────┤
│ ✅ 200 OK — 706 ms                                        │
└──────────────────────────────────────────────────────────┘
```

---

## ⌨️ Keyboard Shortcuts

| Key | Action |
|---|---|
| `Alt + M` | Cycle HTTP method forward (GET → POST → PUT → …) |
| `Alt + Shift + M` | Cycle HTTP method backward |
| `Alt + Enter` | **Send request** |
| `Alt + S` | Save current request to collections |
| `Alt + R` | Switch to Request tab |
| `Alt + E` | Switch to Response tab |
| `Alt + L` | Switch to Collections tab |
| `Alt + N` | Next field (URL → Headers → Body) |
| `Alt + P` | Previous field |
| `Tab` / `Shift + Tab` | Move between URL / Headers / Body fields |
| `↑` / `↓` | Scroll response (in Response tab) |
| `PgUp` / `PgDn` | Fast scroll response |
| `g` / `Home` | Jump to top of response |
| `↑` / `↓` | Navigate collections (in Collections tab) |
| `Enter` | Load selected collection |
| `d` / `Delete` | Delete selected collection |
| `Ctrl + C` | Quit |

---

## 🧪 Quick Test

Try these requests out of the box:

| Method | URL |
|---|---|
| GET | https://httpbin.org/get |
| POST | https://httpbin.org/post |
| PUT | https://httpbin.org/put |
| DELETE | https://httpbin.org/delete |
| PATCH | https://httpbin.org/patch |

### Example: POST with JSON body

1. Press `Alt + R` to go to Request tab
2. Press `Alt + M` until method is `POST`
3. Set URL to `https://httpbin.org/post`
4. Press `Tab` twice to reach the Body field
5. Type: `{"name": "flame", "version": "0.1.0"}`
6. Press `Alt + Enter`

---

## 🔧 Variables

You can use `{{variable_name}}` placeholders anywhere in the URL, headers, or body:

```
URL: {{base_url}}/get
```

The default variable is:

```
base_url = https://httpbin.org
```

Variables are defined in `src/app.rs` in the `App::new()` function.

---

## 💾 Collections

Saved requests are stored in `data/collections.toml`:

```toml
[[requests]]
name = "GitHub Users API"
method = "GET"
url = "https://api.github.com/users/rust-lang"
headers = "Accept: application/vnd.github+json"
body = ""
```

Press `Alt + S` to save the current request, and `Alt + L` to browse your collections.

---

## 🏗️ Project Structure

```
flame/
├── Cargo.toml              # Dependencies
├── README.md
├── LICENSE
├── install.ps1             # Windows installer
├── install.sh              # Linux/macOS installer
├── .github/
│   └── workflows/
│       └── release.yml     # Auto-build releases
├── data/
│   └── collections.toml    # Saved requests
└── src/
    ├── main.rs             # Entry point + event loop
    ├── app.rs              # Application state
    ├── events.rs           # Keyboard input handling
    ├── config.rs           # Colors + settings
    ├── http/
    │   ├── mod.rs          # Public HTTP interface
    │   └── client.rs       # reqwest logic
    ├── storage/
    │   ├── mod.rs          # Storage interface
    │   └── collections.rs  # TOML read/write
    └── ui/
        ├── mod.rs          # Layout composition
        ├── request_view.rs # Request panel
        ├── response_view.rs# Response panel
        └── sidebar.rs      # Collections + History
```

---

## 📚 Tech Stack

| Crate | Purpose |
|---|---|
| [`tokio`](https://crates.io/crates/tokio) | Async runtime |
| [`reqwest`](https://crates.io/crates/reqwest) | HTTP client |
| [`ratatui`](https://crates.io/crates/ratatui) | TUI framework |
| [`crossterm`](https://crates.io/crates/crossterm) | Terminal backend |
| [`serde`](https://crates.io/crates/serde) + [`serde_json`](https://crates.io/crates/serde_json) | JSON handling |
| [`toml`](https://crates.io/crates/toml) | Collections file |
| [`clap`](https://crates.io/crates/clap) | CLI argument parsing |
| [`anyhow`](https://crates.io/crates/anyhow) | Error handling |
| [`chrono`](https://crates.io/crates/chrono) | Timestamps |

---

## 🗺️ Roadmap

- [x] Basic request/response
- [x] 7 HTTP methods
- [x] JSON pretty-print
- [x] Collections (TOML)
- [x] History
- [x] Variables `{{name}}`
- [x] Response scrolling
- [x] Delete collections
- [ ] Cursor movement inside fields
- [ ] Environment files (.env style)
- [ ] Postman collection import
- [ ] File upload (multipart/form-data)
- [ ] GraphQL support

---

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.

---

## 👤 Author

**Fares Mouhoubi**

- GitHub: [@Firefares2005](https://github.com/Firefares2005)

---

<p align="center">
  Made with ❤️ and 🦀 Rust
</p>