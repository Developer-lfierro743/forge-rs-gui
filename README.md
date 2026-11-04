# Forge.rs GUI

In-game mod manager for Horizon. Built as a native `.hnmod` using `horizon-mixins`.

## Structure

```
forge-rs-gui/
├── horizon-mixin.toml          ← manifest with injection points
└── mixins/forge_rs_gui/
    ├── runtime.rs              ← mod state (thread_local!)
    ├── listed.rs               ← Options > Mods entry
    ├── hud.rs                  ← main GUI drawing
    ├── frame.rs                ← per-frame updates
    ├── command.rs              ← chat commands (/forge-rs)
    └── settings.rs             ← settings page
```

## Hook points

| Hook | Purpose |
|------|---------|
| `native_mods` | Shows "Forge.rs GUI" in Options > Mods |
| `native_hud` | Draws the GUI panel when open |
| `native_frame` | Animations, background registry polling |
| `native_command` | `/forge-rs` and `/fr` commands |
| `native_settings` | Settings page entry |

## Build

This mod is meant to be compiled by the Horizon launcher. To build locally:

```bash
# From the Horizon launcher or with forge-rs CLI:
forge-rs build /path/to/forge-rs-gui --output forge_rs_gui.hnmod
```

Or with Horizon's own tools:

```bash
cd /path/to/horizon
python3 native/tools/native.py package /path/to/forge-rs-gui
```

## Install

Place `forge_rs_gui.hnmod` in your Horizon `mods/` folder and launch the game. The first run shows a trust notice.

## Usage

- `/forge-rs` or `/fr` — open/close the GUI
- UP/DOWN — select mods
- ENTER — toggle enable/disable
- ESC — close

## Status

Starter scaffold. The real implementation needs:
- File I/O to read `mods/enabled/` and `mods/disabled/`
- Registry browser with download
- Conflict detection UI
- Dependency resolver display
