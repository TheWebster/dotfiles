use penrose::{
    builtin::actions::key_handler, core::bindings::KeyEventHandler, util::spawn_with_args, x::XConn,
};

use crate::theme::FONT;

static OPTS: &[&str] = &["-p", "$ ", "--fn", FONT];

pub fn launcher<X: XConn>() -> Box<dyn KeyEventHandler<X>> {
    key_handler(|_, _| spawn_with_args("bemenu-run", OPTS))
}
