// Settings page for Forge.rs GUI
fn forge_rs_settings(
    game: &mut crate::game::Game<'_>,
    mod_id: &str,
    buttons: &mut Vec<String>,
) {
    if mod_id == "forge_rs_gui" {
        buttons.push("Open Forge.rs GUI".to_string());
        buttons.push("Open Mods Directory".to_string());
        buttons.push("Check for Updates".to_string());
    }
}
