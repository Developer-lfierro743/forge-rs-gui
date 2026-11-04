// Settings page for Forge.rs GUI
fn forge_rs_settings(game: &mut crate::game::Game<'_>) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        // Stub: in a real mod this would draw a settings page with toggles for:
        // - mods directory path
        // - auto-check for updates
        // - theme colors
        // - registry URL
    });
}
