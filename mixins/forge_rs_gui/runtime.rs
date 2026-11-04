thread_local! {
    pub static STATE: ForgeRsGuiState = ForgeRsGuiState::new();
}

#[derive(Debug, Clone)]
pub struct ForgeRsGuiState {
    pub open: bool,
    pub selected_index: usize,
    pub installed_mods: Vec<crate::game::mod_api::NativeMod>,
    pub toast: Option<(String, std::time::Instant)>,
}

impl ForgeRsGuiState {
    pub fn new() -> Self {
        Self {
            open: false,
            selected_index: 0,
            installed_mods: Vec::new(),
            toast: None,
        }
    }

    pub fn show_toast(&mut self, msg: impl Into<String>) {
        self.toast = Some((msg.into(), std::time::Instant::now()));
    }
}
