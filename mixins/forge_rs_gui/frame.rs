// Per-frame update for Forge.rs GUI
fn forge_rs_frame(
    game: &mut crate::game::Game<'_>,
    elapsed: std::time::Duration,
) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }

        // Poll registry in background if empty
        if state.registry_mods.is_empty() && state.last_poll.elapsed() > std::time::Duration::from_secs(5) {
            state.last_poll = std::time::Instant::now();
            // In a real mod: spawn a thread or use async runtime to fetch registry
        }

        // Fade toast after 3s
        if let Some((_, born)) = &state.toast {
            if born.elapsed() > std::time::Duration::from_secs(3) {
                state.toast = None;
            }
        }
    });
}
