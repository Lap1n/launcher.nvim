use mlua::prelude::LuaFunction;
use mlua::prelude::LuaResult;
use mlua::prelude::LuaTable;
use mlua::Lua;
use mlua::TableExt;
use mlua::ToLua;
use mlua::{Error, LuaSerdeExt, Result, UserData, Value};
use nvim_oxi::print;

use mlua::Table;
use serde::{Deserialize, Serialize};

use crate::launcher::ConfigurationConfig;

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    lua.load("require('dap')").eval()
}
pub fn run(lua: &Lua, config: ConfigurationConfig) -> LuaResult<()> {
    let opts = lua.create_table();
    get(lua)?.get::<_, LuaTable>("run")?.call((config, opts))?;
    Ok(())
}
