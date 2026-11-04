// Options > Mods list entry for Forge.rs GUI
fn forge_rs_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::forge_rs_gui as fgui;
    fgui::STATE.with(|state| {
        mods.push(NativeMod {
            id: "forge_rs_gui".into(),
            name: "Forge.rs GUI".into(),
            version: env!("CARGO_PKG_VERSION").into(),
            description: "In-game mod manager for Horizon".into(),
            enabled: true,
        });
    });
}
