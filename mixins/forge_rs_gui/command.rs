// Chat commands for Forge.rs GUI
fn forge_rs_command(
    game: &mut crate::game::Game<'_>,
    command: &str,
    ran: &mut bool,
) {
    use crate::game::forge_rs_gui as fgui;
    let cmd = command.trim();
    if cmd == "/forge-rs" || cmd == "/fr" {
        fgui::STATE.with(|state| {
            state.open = !state.open;
            if state.open {
                state.show_toast("Forge.rs GUI opened");
            }
        });
        *ran = true;
    } else if cmd.starts_with("/fr ") || cmd.starts_with("/forge-rs ") {
        fgui::STATE.with(|state| {
            state.show_toast("Coming soon: full registry browser");
        });
        *ran = true;
    }
}
