# 🧙 Shaman DE

**Agentic Desktop Environment**  
No windows, no workspaces. Just a grid of activities controlled by a local AI.

## ✨ Vision

Shaman rethinks how we interact with a PC. Instead of overlapping windows and virtual desktops, you get a **bento‑box grid** – resizable, draggable tiles that snap to an organic layout. Every tile can host a terminal, a system monitor, a web view, your notes, or any custom widget.

And when you don’t want to click or type commands, just **talk to the built‑in AI agent**. Say:

> *“Move the terminal next to the system monitor.”*  
> *“Open Firefox and split the grid into two columns.”*  
> *“What did I write in my Obsidian note about the API design?”*

The agent runs **locally** (via Ollama or llama.cpp) – no cloud, no privacy leaks.

## 🚀 Features

- **Bento‑box grid** – draggable, resizable tiles that snap to a dynamic cell layout.
- **Local AI agent** – optional, with fine‑grained permissions (web, files, commands).
- **Obsidian vault watcher** – uses `inotify` to index your markdown files in real time.
- **Persistent layout** – saves positions and sizes to `~/.config/jobshaman/grid/`.
- **Written in Rust** – fast, safe, and memory‑efficient.

## 🛠️ Installation

### Dependencies

- Linux (Wayland) – developed on Arch + Hyprland, but works on any desktop.
- [Ollama](https://ollama.com/) (optional) – for the AI backend.
- `libinotify` – usually preinstalled on Linux.
- Rust toolchain – install via [rustup](https://rustup.rs/).

### Build from source

```bash
git clone https://github.com/yourusername/jobshaman-grid.git
cd jobshaman-grid
cargo build --release
