// Draw the Forge.rs GUI screen when open
fn forge_rs_hud(game: &mut crate::game::Game<'_>, batch: &mut crate::game::HudBatch) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }

        // Simple modal background
        let w = 400.0f32;
        let h = 300.0f32;
        let x = (game.width() - w) / 2.0;
        let y = (game.height() - h) / 2.0;

        // Draw panel
        crate::game::native_hooks::helpers::fill_rect(batch, x, y, w, h, [0.1, 0.1, 0.1, 0.95]);
        crate::game::native_hooks::helpers::stroke_rect(batch, x, y, w, h, [0.4, 0.4, 0.4, 1.0]);

        // Title
        let title = "Forge.rs";
        crate::game::native_hooks::helpers::draw_text(batch, x + 10.0, y + 10.0, title, [1.0, 0.76, 0.2, 1.0], 20.0);

        // Draw mods list
        let mut yy = y + 40.0;
        for (i, m) in state.registry_mods.iter().enumerate() {
            if i == state.selected_index {
                crate::game::native_hooks::helpers::fill_rect(batch, x + 5.0, yy - 2.0, w - 10.0, 20.0, [0.3, 0.3, 0.3, 0.8]);
            }
            let label = format!("{} v{}", m.id, m.version);
            crate::game::native_hooks::helpers::draw_text(batch, x + 10.0, yy, &label, [1.0, 1.0, 1.0, 1.0], 14.0);
            yy += 20.0;
        }

        // Instructions
        let help = "UP/DOWN = select  ENTER = toggle  ESC = close";
        crate::game::native_hooks::helpers::draw_text(batch, x + 10.0, y + h - 25.0, help, [0.7, 0.7, 0.7, 1.0], 12.0);
    });
}
