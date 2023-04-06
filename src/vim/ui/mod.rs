use mlua::prelude::LuaFunction;
use mlua::prelude::LuaResult;
use mlua::prelude::LuaTable;
use mlua::Lua;
pub fn get(lua: &Lua) -> LuaResult<LuaTable> {
    super::get(lua)?.get::<_, LuaTable>("ui")
}
pub fn select(
    lua: &Lua,
    choices: LuaTable,
    options: LuaTable,
    callback: LuaFunction,
) -> LuaResult<()> {
    get(lua)?
        .get::<_, LuaFunction>("select")?
        .call((choices, options, callback))
}
