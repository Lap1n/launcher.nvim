use nvim_oxi::{self as oxi, Dictionary, Function, Object};

mod dap;
mod launcher;
mod vim;

mod prelude {
    pub use crate::launcher::load_json;
    pub use crate::launcher::run;
    pub use crate::launcher::LaunchConfig;
}
use crate::prelude::*;

fn launch(_: String) -> oxi::Result<i32> {
    let path = ".vscode/launch.json";
    run(path, false)?;
    return Ok(0);
}
fn debug(_: String) -> oxi::Result<i32> {
    let path = ".vscode/launch.json";
    run(path, true)?;
    return Ok(0);
}

#[oxi::module]
fn launcher() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([
        ("run", Object::from(Function::from_fn(launch))),
        ("debug", Object::from(Function::from_fn(debug))),
    ]))
}
