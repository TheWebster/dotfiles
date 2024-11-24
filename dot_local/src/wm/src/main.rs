mod util;

mod be_menu;
mod bindings;
mod config;
mod mouse;
mod theme;

use std::{env::var, sync::LazyLock};

use bindings::{key_bindings, mouse_bindings};
use penrose::{
    builtin::layout::MainAndStack,
    core::{Config, WindowManager},
    extensions::hooks::add_ewmh_hooks,
    pure::Stack,
    x11rb::RustConn,
    Color, Result,
};
use tracing_subscriber::{filter::LevelFilter, fmt, EnvFilter};

pub static SCALE: LazyLock<u32> =
    LazyLock::new(|| var("GDK_SCALE").map_or(1, |v| v.parse().unwrap_or(1)));

fn main() -> Result<()> {
    fmt()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .init();

    let mut config = Config {
        normal_border: Color::new_from_hex(0x000000ff),
        focused_border: Color::new_from_hex(0xff0000ff),
        border_width: 1 * SCALE.clone(),
        focus_follow_mouse: true,
        default_layouts: Stack::try_from_iter([MainAndStack::boxed_default()])
            .expect("empty layout stack"),
        tags: (1..10).map(|n| n.to_string()).collect(),
        floating_classes: Vec::new(),
        startup_hook: None,
        event_hook: None,
        manage_hook: None,
        refresh_hook: None,
        layout_hook: None,
    };
    // let mut config = Config::default();

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
