use mlua::Lua;

use mlua::prelude::LuaResult;
use mlua::prelude::LuaTable;
pub mod ui;

pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    lua.globals().get("vim")
}
