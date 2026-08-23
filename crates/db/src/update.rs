use sqlx::{query, query_scalar, types::Uuid};

/// Updates player's name and/or color_hex
pub async fn update_player_by_uuid(
    manager: &crate::Manager,
    uuid: Uuid,
    name: Option<&str>,
    color_hex: Option<&str>,
) -> anyhow::Result<()> {
    let mut tx = manager.pool.begin().await?;

    let exists = query_scalar!("select count(*) from players where uuid = $1", uuid)
        .fetch_one(&mut *tx)
        .await?
        .unwrap()  // it cant be none with `count()`, but it could be none with `max()`
        > 0;

    if !exists {
        return Err(anyhow::Error::msg(format!(
            "Player with UUID {} does not exist in the database",
            uuid
        )));
    }

    if let Some(name) = name {
        query!("update players set name = $1 where uuid = $2", name, uuid)
            .execute(&mut *tx)
            .await?;
    }

    if let Some(color_hex) = color_hex {
        query!(
            "update players set color_hex = $1 where uuid = $2",
            color_hex,
            uuid
        )
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    Ok(())
}
