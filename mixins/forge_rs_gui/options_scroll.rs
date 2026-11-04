// Handle scroll wheel in mod manager
fn forge_rs_options_scroll(
    game: &mut crate::game::Game<'_>,
    amount: f32,
) -> ::core::ops::ControlFlow<Result<()>> {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return ::core::ops::ControlFlow::Continue(Ok(()));
        }
        state.scroll += amount;
        ::core::ops::ControlFlow::Break(Ok(()))
    })
}
