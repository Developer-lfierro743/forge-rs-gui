// Draw the Forge.rs GUI screen when open
// Hook: native_hud(&mut self, batch: &mut HudBatch, corners: &mut Corners)
// Helper receives: batch, corners (no self/game)
fn forge_rs_hud(
    batch: &mut crate::hud::HudBatch,
    _corners: &mut crate::extras_hud::Corners,
) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }

        let w = 420.0f32;
        let h = 320.0f32;
        let x = (batch.width - w) / 2.0;
        let y = (batch.height - h) / 2.0;

        // Simple panel using text labels
        // A full impl would use native_fill via batch, but that needs
        // a Game reference which hook-point helpers don't receive.
        // For now this proves the hook wiring works.
        crate::game::native_hooks::helpers::draw_text(batch, "Forge.rs", x + 12.0, y + 12.0, 0xFFD7991E);
        crate::game::native_hooks::helpers::draw_text(batch, "GUI placeholder - full impl coming soon", x + 12.0, y + 40.0, 0xFFAAAAAA);
    });
}
