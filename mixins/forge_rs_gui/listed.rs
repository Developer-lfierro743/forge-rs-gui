fn forge_rs_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        mods.push(crate::game::native_hooks::NativeMod {
            id: "forge_rs_gui".into(),
            title: "Forge.rs GUI".into(),
            description: "In-game mod manager for Horizon".into(),
            keys: vec![
                ["forge_rs_gui.toggle".into(), "Toggle Forge.rs GUI".into(), "key.keyboard.g".into()],
            ],
        });
    });
}
