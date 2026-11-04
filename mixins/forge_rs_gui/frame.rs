fn forge_rs_frame(elapsed: std::time::Duration) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }
        // Fade toast after 3s
        if let Some((_, born)) = &state.toast {
            if born.elapsed() > std::time::Duration::from_secs(3) {
                state.toast = None;
            }
        }
    });
}
