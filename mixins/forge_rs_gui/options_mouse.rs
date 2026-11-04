// Handle mouse input in mod manager
fn forge_rs_options_mouse(
    game: &mut crate::game::Game<'_>,
    pressed: bool,
    right: bool,
) -> ::core::ops::ControlFlow<Result<()>> {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        if !state.open {
            return ::core::ops::ControlFlow::Continue(Ok(()));
        }
        // Stub: real impl would check clicks against list items
        // and call toggle_mod() / open settings, etc.
        ::core::ops::ControlFlow::Break(Ok(()))
    })
}
