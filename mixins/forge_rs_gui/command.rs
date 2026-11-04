// Chat commands for Forge.rs GUI
fn forge_rs_command(game: &mut crate::game::Game<'_>, command: &str, ran: &mut bool) {
    use crate::game::forge_rs_gui as fgui;
    let cmd = command.trim();
    if cmd == "/forge-rs" || cmd == "/fr" {
        fgui::STATE.with(|state| {
            state.open = !state.open;
            state.show_toast(if state.open { "Forge.rs opened" } else { "Forge.rs closed" });
        });
        *ran = true;
    } else if cmd.starts_with("/forge-rs ") || cmd.starts_with("/fr ") {
        let sub = cmd.split_whitespace().nth(1).unwrap_or("");
        fgui::STATE.with(|state| {
            match sub {
                "list" => {
                    state.show_toast("Use the GUI to browse mods");
                }
                "install" => {
                    state.show_toast("Use the GUI to install mods");
                }
                _ => {
                    state.show_toast("Usage: /fr [list|install|toggle]");
                }
            }
        });
        *ran = true;
    }
}
