fn forge_rs_command(command: &str, ran: &mut bool) {
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
    }
}
