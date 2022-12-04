mod io;
mod module;

use Werilog::prelude::*;

pub fn convert(input: &str) -> String {
    let ret = single_module_parser().run(input);
    match ret {
        Ok(ast) => module::module(ast),
        Err(_) => format!("{:#?}", ret)
    }
}
