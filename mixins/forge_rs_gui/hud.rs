// Minimal HUD hook - shows a small indicator when mod manager is open
fn forge_rs_hud(
    batch: &mut crate::hud::HudBatch,
    _corners: &mut crate::extras_hud::Corners,
) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }
        // Small indicator in top-right corner
        crate::game::native_hooks::helpers::draw_text(
            batch,
            "Forge.rs",
            batch.width - 80.0,
            10.0,
            0xFFD7991E,
        );
    });
}
