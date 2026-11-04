// Handle title button clicks
fn forge_rs_title_click(
    title: &mut Title,
    x: f32,
    y: f32,
    size: (f32, f32),
) -> ::core::ops::ControlFlow<(Action, bool)> {
    use core::ops::ControlFlow;
    use crate::game::forge_rs_gui as fgui;

    if !matches!(title.mode, Mode::Main) {
        return ControlFlow::Continue(());
    }
    let (w, h) = (size.0.ceil(), size.1.ceil());
    let hit = title
        .buttons(w, h)
        .into_iter()
        .find(|b| b.enabled && x >= b.x && x < b.x + b.w && y >= b.y && y < b.y + 20.0)
        .map(|b| b.id);
    if hit == Some(42) {
        fgui::STATE.with(|state| {
            state.open = true;
            state.selected_index = 0;
            state.scroll = 0.0;
        });
        // Open Options screen so our draw hook can take it over
        return ControlFlow::Break((Action::Options, true));
    }
    ControlFlow::Continue(())
}
