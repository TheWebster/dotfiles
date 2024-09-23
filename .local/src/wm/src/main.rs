mod util;

mod be_menu;
mod bindings;
mod config;
mod mouse;
mod theme;

use bindings::{key_bindings, mouse_bindings};
use penrose::{
    core::{Config, WindowManager},
    extensions::hooks::add_ewmh_hooks,
    x11rb::RustConn,
    Result,
};
use std::collections::HashMap;
use tracing_subscriber::{filter::LevelFilter, fmt, EnvFilter};

fn main() -> Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .init();

    // let mut config = Config {
    //     normal_border: Color::new_from_hex(0x000000ff),
    //     focused_border: Color::new_from_hex(0xff0000ff),
    //     border_width: 1,
    //     focus_follow_mouse: true,
    //     default_layouts: Stack::try_from_iter([MainAndStack::boxed_default()])
    //         .expect("empty layout stack"),
    //     tags: Vec::new(),
    //     floating_classes: Vec::new(),
    //     startup_hook: None,
    //     event_hook: None,
    //     manage_hook: None,
    //     refresh_hook: None,
    //     layout_hook: None,
    // };
    let mut config = Config::default();

    config = add_ewmh_hooks(config);
    let keys = key_bindings().expect("parse keybindings");
    let conn = RustConn::new().expect("X Connection");
    let wm =
        WindowManager::new(config, keys, mouse_bindings(), conn).expect("Create Windowmanager");

    wm.run()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bindings_parse_correctly_with_xmodmap() {
        key_bindings().expect("generate key bindings");
    }
}
