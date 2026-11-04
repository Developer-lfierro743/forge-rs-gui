// Add Forge.rs button to title screen
fn forge_rs_title_buttons(title: &Title, w: f32, h: f32) -> ::core::ops::ControlFlow<Vec<Button>> {
    use core::ops::ControlFlow;
    use crate::game::forge_rs_gui as fgui;

    let mut buttons = title.buttons(w, h);
    if matches!(title.mode, Mode::Main) {
        // Find the Accessibility button (id=6) and insert after it
        if let Some(pos) = buttons.iter().position(|b| b.id == 6) {
            let b = &buttons[pos];
            buttons.insert(pos + 1, Button {
                x: b.x + b.w + 4.0,
                y: b.y,
                w: 80.0,
                label: "Mods".into(),
                enabled: true,
                id: 42,
                icon: None,
                trailing: None,
            });
        }
    }
    ControlFlow::Break(buttons)
}
