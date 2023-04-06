use mlua::{Error, LuaSerdeExt, Result, UserData, Value};
use mlua::{Lua, ToLua};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PresentationConfig {
    hidden: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigurationConfig {
    pub name: String,
    #[serde(rename = "type")]
    pub run_type: String,
    pub request: String,
    pub program: String,
    pub args: Vec<String>,
}
impl ToLua<'_> for ConfigurationConfig {
    fn to_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table()?;
        table.set("type", self.run_type)?;
        table.set("request", self.request)?;
        table.set("name", self.name)?;
        table.set("program", self.program)?;
        table.set("args", self.args)?;
        Ok(Value::Table(table))
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchConfig {
    pub version: String,
    pub configurations: Vec<ConfigurationConfig>,
}
