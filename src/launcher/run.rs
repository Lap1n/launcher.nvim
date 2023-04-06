use crate::dap;
use crate::vim;
use oxi::libuv::AsyncHandle;
use tokio::sync::mpsc;

use super::{load_json, process::launch_process, ConfigurationConfig};
use nvim_oxi::{self as oxi};

use std::thread;

use mlua::prelude::LuaFunction;
fn vim_ui_select(items: &Vec<String>, callback: LuaFunction) -> oxi::Result<()> {
    let lua = oxi::mlua::lua();
    let choices = lua.create_table()?;
    for (i, item) in items.iter().enumerate() {
        choices.set(i + 1, item.as_str())?;
    }

    let options = lua.create_table()?;
    options.set("prompt", "select")?;

    vim::ui::select(&lua, choices, options, callback)?;
    Ok(())
}

pub fn run(path: &str, debug: bool) -> oxi::Result<()> {
    let json = match load_json(path) {
        Ok(json) => json,
        Err(error) => {
            print!("Error loading json: {}", error);
            return Ok(());
        }
    };
    let configs = json.configurations;
    let mut items = Vec::new();
    for config in &configs {
        items.push(config.name.clone());
    }
    let lua = oxi::mlua::lua();

    let callback = lua.create_function(move |_, (_, idx): (String, usize)| {
        let (sender, mut receiver) = mpsc::unbounded_channel::<(usize, String)>();
        let zero_idx = idx - 1;
        let config = configs[zero_idx].clone();
        if debug {
            dap::run(&lua, config)?;
        } else {
            oxi::api::command("split").unwrap();
            let mut buffer = oxi::api::create_buf(true, true).unwrap();
            oxi::api::set_current_buf(&buffer).unwrap();
            buffer.set_option("modifiable", false).unwrap();
            let handle = AsyncHandle::new(move || {
                let (count, output) = receiver.blocking_recv().unwrap();
                let mut buffer = buffer.clone();
                oxi::schedule(move |()| {
                    buffer.set_option("modifiable", true).unwrap();
                    buffer.set_lines(count..count + 1, false, [format!("{output}")])?;
                    buffer.set_option("modifiable", false).unwrap();
                    Ok(())
                });
                Ok::<_, oxi::Error>(())
            })
            .unwrap();
            let _ = thread::spawn(move || launch_process(config, handle, sender));
        }
        Ok(())
    })?;
    vim_ui_select(&items, callback)?;
    return Ok(());
}
