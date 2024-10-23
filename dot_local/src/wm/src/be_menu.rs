use penrose::{
    builtin::actions::key_handler, core::bindings::KeyEventHandler, util::spawn_with_args, x::XConn,
};

use crate::{
    theme::{FONT, FONT_SIZE},
    SCALE,
};

pub fn launcher<X: XConn>() -> Box<dyn KeyEventHandler<X>> {
    let font = String::from_iter([FONT.into(), format!(" {}", SCALE.clone() * FONT_SIZE)]);

    key_handler(move |_, _| spawn_with_args("bemenu-run", &["-p", ">> ", "--fn", &font]))
}
