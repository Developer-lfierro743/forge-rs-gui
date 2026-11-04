// Forge.rs GUI — runtime state
// This file becomes crate::game::forge_rs_gui inside horizon-client.

thread_local! {
    pub static STATE: ForgeRsGuiState = ForgeRsGuiState::new();
}

#[derive(Debug, Clone)]
pub struct ForgeRsGuiState {
    pub open: bool,
    pub mods_dir: String,
    pub selected_index: usize,
    pub scroll: f32,
    pub registry_mods: Vec<crate::game::mod_api::NativeMod>,
    pub installed_mods: Vec<crate::game::mod_api::NativeMod>,
    pub last_poll: std::time::Instant,
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
            installed_mods: Vec::new(),
            last_poll: std::time::Instant::now(),
            toast: None,
        }
    }

    pub fn show_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), std::time::Instant::now()));
    }
}
