# Local Fonts Agent

A lightweight cross-platform desktop application that serves system fonts via HTTP API.

![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF) ![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Features

- 🎨 **Font Discovery**: Automatically scans and catalogs all system fonts
- 📊 **Font Metadata**: Provides detailed information (family, weight, style, etc.)
- 🖼️ **SVG Previews**: Generates vector previews for each font
- 🌐 **HTTP API**: Serves fonts and metadata via REST endpoints
- 🚀 **Auto-start**: Optionally launch on system login
- 💾 **Caching**: Builds and serves cached font data for fast access
- 📝 **Logging**: Structured logging with daily rotation
- 🖥️ **Cross-platform**: Works on macOS, Windows, and Linux

## API Endpoints

### Base URL
`http://localhost:36687`

### Available Endpoints

| Method | Endpoint | Description | Response |
|--------|----------|-------------|----------|
| `GET` | `/` | Health check | `"OK"` (text/plain) |
| `GET` | `/v1/fonts-meta.json` | Get metadata for all fonts | Array of font metadata objects (application/json) |
| `GET` | `/v1/fonts-preview.json` | Get SVG previews for all fonts | Object mapping postscript names to SVG strings (application/json) |
| `GET` | `/v1/fonts/{postscript_name}` | Download specific font file | Font file binary (font/ttf, font/otf, etc.) |
| `POST` | `/v1/refresh` | Rebuild font cache | 204 No Content on success, 500 on error |

#### Response Structures

**GET /v1/fonts-meta.json**
```json
[
  {
    "family": "Helvetica",
    "full_name": "Helvetica Regular",
    "postscript_name": "Helvetica",
    "is_monospace": false,
    "weight": 400.0,
    "style": "Normal",
    "stretch": 1.0
  }
]
```

**GET /v1/fonts-preview.json**
```json
{
  "Helvetica": "<svg viewBox=\"...\">...</svg>"
}
```


## Usage

1. Launch the application (runs in system tray)
2. Access fonts via HTTP API at `http://localhost:36687`
3. Use tray menu to:
   - Toggle "Start on Login"
   - Quit the application

## Logs

Logs are stored in platform-specific directories:

- **macOS**: `~/Library/Logs/sh.karthikeyan.local-fonts-agent/`
- **Linux**: `~/.local/share/local-fonts-agent/logs/`
- **Windows**: `%APPDATA%\local-fonts-agent\logs\`

## Cache

Font cache is stored at:

- **macOS**: `~/Library/Caches/sh.karthikeyan.local-fonts-agent/static/`
- **Linux**: `~/.cache/local-fonts-agent/static/`
- **Windows**: `%LOCALAPPDATA%\local-fonts-agent\cache\static\`
