// Forge.rs GUI — runtime state
// This file becomes crate::game::forge_rs_gui inside horizon-client.

use crate::game::native_hooks::NativeMod;

thread_local! {
    pub static STATE: crate::game::forge_rs_gui::ForgeRsGuiState = crate::game::forge_rs_gui::ForgeRsGuiState::new();
}

pub struct ForgeRsGuiState {
    pub open: bool,
    pub mods_dir: String,
    pub selected_index: usize,
    pub scroll: f32,
    pub registry_mods: Vec<NativeMod>,
    pub last_frame: std::time::Instant,
    pub toast: Option<(String, std::time::Instant)>,
}

impl ForgeRsGuiState {
    pub fn new() -> Self {
        Self {
            open: false,
            mods_dir: "./mods".to_string(),
            selected_index: 0,
            scroll: 0.0,
            registry_mods: Vec::new(),
            last_frame: std::time::Instant::now(),
            toast: None,
        }
    }

    pub fn show_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), std::time::Instant::now()));
    }
}

// Helper to read installed mods from disk.
// In a real mod this would parse the mods/enabled and mods/disabled dirs.
pub fn installed_mods() -> Vec<NativeMod> {
    let mut out = Vec::new();
    // Stub: in a real build the launcher passes the actual mods list through a binding
    // or the mod reads the mods dir directly. This is a placeholder.
    out
}

pub fn toggle_mod(_id: &str) {
    // Stub: move .hnmod between mods/enabled and mods/disabled
}

pub fn install_from_registry(_url: &str) {
    // Stub: download .hnmod, verify, copy into mods/enabled
}
