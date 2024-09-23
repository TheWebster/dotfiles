use penrose::{
    builtin::{
        actions::{exit, floating::sink_focused, modify_with, send_layout_message, spawn},
        layout::messages::{ExpandMain, IncMain, ShrinkMain},
    },
    core::bindings::{
        click_handler, parse_keybindings_with_xmodmap, KeyBindings, ModifierKey, MouseBindings,
        MouseButton, MouseState,
    },
    map,
    x11rb::RustConn,
    Result,
};

use crate::{
    be_menu::launcher,
    mouse::{drag_handler, resize_handler},
};

pub fn key_bindings() -> Result<KeyBindings<RustConn>> {
    let mut raw_bindings = map! {
        map_keys: |k: &str| k.to_string();

        "M-j" => modify_with(|cs| cs.focus_down()),
        "M-k" => modify_with(|cs| cs.focus_up()),
        "M-S-j" => modify_with(|cs| cs.swap_down()),
        "M-S-k" => modify_with(|cs| cs.swap_up()),
        "M-q" => modify_with(|cs| cs.kill_focused()),
        "M-S-Up" => send_layout_message(|| IncMain(1)),
        "M-S-Down" => send_layout_message(|| IncMain(-1)),
        "M-S-Right" => send_layout_message(|| ExpandMain),
        "M-S-Left" => send_layout_message(|| ShrinkMain),
        "M-Return" => launcher(),
        "M-S-Return" => spawn("alacritty"),
        "M-S-Escape" => exit(),
    };

    for tag in &["1", "2", "3", "4", "5", "6", "7", "8", "9"] {
        raw_bindings.extend([
            (format!("M-{tag}"), modify_with(move |cs| cs.focus_tag(tag))),
            (
                format!("M-S-{tag}"),
                modify_with(move |cs| cs.move_focused_to_tag(tag)),
            ),
        ]);
    }

    parse_keybindings_with_xmodmap(raw_bindings)
}

pub fn mouse_bindings() -> MouseBindings<RustConn> {
    map! {
        map_keys: |(button, modifiers)| MouseState {button, modifiers};

        (MouseButton::Left, vec![ModifierKey::Meta]) => drag_handler(),
        (MouseButton::Right, vec![ModifierKey::Meta]) => resize_handler(),
        (MouseButton::Middle, vec![ModifierKey::Meta]) => click_handler(sink_focused()),
    }
}
