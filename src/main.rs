#[macro_use]
extern crate failure;

use clap::{App, Arg};

use crate::cmd::cmd_papk::cmd_papk;
use crate::cmd::cmd_pclass::cmd_pclass;
use crate::cmd::cmd_unpack::cmd_unpack;

mod cmd;

mod highlight;

fn main() {
    let papk_opt = Arg::with_name("papk").long("papk").takes_value(true);
    let pclass_opt = Arg::with_name("pclass").long("pclass").takes_value(true);
    let unpack_opt = Arg::with_name("unpack").long("unpack").takes_value(true);

    let app = App::new("decoder")
        .version("0.0.1")
        .arg(papk_opt)
        .arg(pclass_opt)
        .arg(unpack_opt);

    let matches = app.get_matches();

    if let Some(str) = matches.value_of("papk") {
        cmd_papk(String::from(str));
    }

    if let Some(str) = matches.value_of("pclass") {
        cmd_pclass(String::from(str));
    }

    if let Some(str) = matches.value_of("unpack_opt") {
        cmd_unpack(String::from(str));
    }
}
