use mlua::prelude::*;

fn main() -> LuaResult<()> {
    let lua = Lua::new();

    let table = lua.create_table()?;
    table.set(1, "one")?;
    table.set(2, "two")?;
    table.set(3, "three")?;

    lua.globals().set("table", table)?;

    lua.load(
        r#"
        for k,v in pairs(table) do
            print(k,v)
        end
        "#,
    )
    .exec()?;

    Ok(())
}
