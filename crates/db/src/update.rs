use crate::Manager;

/// Update Player based on his/her ID
///
/// Why are parameters `name` and `color` are `Option<Option<&str>>`?
/// - First/Outer `Option` is saying if you want to change the value
/// - Second/Inner `Option` is being inserted to the database
///
/// Examples:
/// - If you want to update name to some value: `Some(Some("Jozko123"))`
/// - If you want to update name to null: `Some(None)`
/// - If you don't want to update name: `None`
///
///
pub async fn update_player(
    manager: &Manager,
    id: i32,
    name: Option<Option<&str>>,
    color: Option<Option<&str>>,
) -> anyhow::Result<()> {
    if name.is_none() && color.is_none() {
        return Ok(());
    }

    // something like `where 1=1`
    let mut builder = crate::Builder::new("update players set id = ");
    builder.push_bind(id);

    if let Some(name) = name {
        builder.push(" and name = ");
        builder.push_bind(name);
    }

    if let Some(color) = color {
        builder.push(" and color = ");
        builder.push_bind(color);
    }

    builder.push(" where id = ");
    builder.push_bind(id);

    builder.build().execute(&manager.pool).await?;

    Ok(())
}
