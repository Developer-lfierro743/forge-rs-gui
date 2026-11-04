fn forge_rs_hud(
    batch: &mut crate::hud::HudBatch,
    _corners: &mut crate::extras_hud::Corners,
) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return;
        }
        crate::game::native_hooks::helpers::draw_text(
            batch,
            "Forge.rs GUI is open",
            batch.width - 180.0,
            10.0,
            0xFFD7991E,
        );
    });
}
