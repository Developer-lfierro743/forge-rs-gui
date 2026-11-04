// Draw Forge.rs mod manager, replacing the vanilla options screen
fn forge_rs_draw_options(
    game: &mut crate::game::Game<'_>,
    batch: &mut HudBatch,
) -> ::core::ops::ControlFlow<()> {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return ::core::ops::ControlFlow::Continue(());
        }

        let w = 500.0f32;
        let h = 360.0f32;
        let x = (batch.width - w) / 2.0;
        let y = (batch.height - h) / 2.0;

        // Background
        game.native_fill(batch, [x, y, w, h], 0xFF1A1A1A);
        game.native_fill(batch, [x, y, w, 2.0], 0xFF444444);
        game.native_fill(batch, [x, y + h - 2.0, w, 2.0], 0xFF444444);
        game.native_fill(batch, [x, y, 2.0, h], 0xFF444444);
        game.native_fill(batch, [x + w - 2.0, y, 2.0, h], 0xFF444444);

        // Title
        game.native_text(batch, "Forge.rs Mod Manager", [x + 16.0, y + 16.0], 0xFFD7991E, true);

        // Close button
        game.native_text(batch, "[Close]", [x + w - 70.0, y + 16.0], 0xFF888888, false);

        // Mod list
        let mut yy = y + 50.0;
        for (i, m) in state.installed_mods.iter().enumerate() {
            if i == state.selected_index {
                game.native_fill(batch, [x + 12.0, yy - 2.0, w - 24.0, 22.0], 0xFF333333);
            }
            let label = format!("{} v{}", m.title, m.id);
            game.native_text(batch, &label, [x + 20.0, yy], 0xFFFFFFFF, false);
            yy += 26.0;
        }

        if state.installed_mods.is_empty() {
            game.native_text(batch, "No mods installed", [x + 20.0, yy], 0xFF888888, false);
        }

        // Instructions
        let help = "UP/DOWN = select  ESC = close";
        game.native_text(batch, help, [x + 16.0, y + h - 30.0], 0xFF888888, false);

        // Cancel vanilla options drawing
        ::core::ops::ControlFlow::Break(())
    })
}
