# Slate

A personal task tracker for the terminal, inspired by [Linear](https://linear.app). Kanban board with vim-style navigation, backed by SQLite.

<!-- ![screenshot](./assets/screenshot.png) -->

## Features

- Three-column kanban board: **Todo**, **In Progress**, **Done**
- Cards with auto-incrementing IDs (`ST-1`, `ST-2`, ...)
- Vim-style keyboard navigation
- SQLite storage (persists at `~/.local/share/slate/kanban.db`)
- Dark theme with rounded card borders

## Install

```
cargo install --git https://github.com/ananddubey/slate
```

Or build from source:

```
git clone https://github.com/ananddubey/slate
cd slate
cargo install --path .
```

## Usage

```
slate
```

### Keybindings

| Key | Action |
|-----|--------|
| `h` / `l` | Switch columns |
| `j` / `k` | Navigate cards |
| `g` / `G` | Jump to first / last card |
| `n` | New issue |
| `m` | Move issue forward (Todo -> In Progress -> Done) |
| `M` | Move issue backward |
| `d` | Delete issue |
| `q` | Quit |

## Tech Stack

- [Ratatui](https://ratatui.rs) - Terminal UI framework
- [rusqlite](https://github.com/rusqlite/rusqlite) - SQLite bindings (bundled)
- [color-eyre](https://github.com/eyre-rs/eyre) - Error handling

## License

MIT
