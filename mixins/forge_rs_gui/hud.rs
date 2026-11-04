// Draw the Forge.rs GUI screen when open
fn forge_rs_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut crate::hud::HudBatch,
    corners: &mut crate::extras_hud::Corners,
) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }

        let w = 420.0f32;
        let h = 320.0f32;
        let x = (game.width() as f32 - w) / 2.0;
        let y = (game.height() as f32 - h) / 2.0;

        // Background panel (dark translucent)
        game.native_fill(batch, [x, y, w, h], 0xFF1A1A1A);

        // Border
        game.native_fill(batch, [x, y, w, 2.0], 0xFF444444);
        game.native_fill(batch, [x, y + h - 2.0, w, 2.0], 0xFF444444);
        game.native_fill(batch, [x, y, 2.0, h], 0xFF444444);
        game.native_fill(batch, [x + w - 2.0, y, 2.0, h], 0xFF444444);

        // Title
        game.native_text(batch, "Forge.rs", [x + 12.0, y + 12.0], 0xFFD7991E, true);

        // Tabs
        let tab_y = y + 38.0;
        game.native_text(batch, "Installed", [x + 12.0, tab_y], 0xFFAAAAAA, false);
        game.native_text(batch, "Registry", [x + 100.0, tab_y], 0xFFAAAAAA, false);

        // Mods list
        let mut yy = y + 60.0;
        for (i, m) in state.installed_mods.iter().enumerate() {
            if i == state.selected_index {
                game.native_fill(batch, [x + 8.0, yy - 2.0, w - 16.0, 22.0], 0xFF333333);
            }
            let label = format!("{}", m.title);
            game.native_text(batch, &label, [x + 14.0, yy], 0xFFFFFFFF, false);
            yy += 24.0;
        }

        if state.installed_mods.is_empty() {
            game.native_text(batch, "No mods installed", [x + 14.0, yy], 0xFF888888, false);
            yy += 24.0;
        }

        // Instructions
        let help = "UP/DOWN = select  ENTER = toggle  ESC = close";
        game.native_text(batch, help, [x + 12.0, y + h - 28.0], 0xFF888888, false);
    });
}
