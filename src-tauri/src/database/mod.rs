use sqlx::sqlite::SqlitePool;

pub async fn init_db(pool: SqlitePool) -> Result<(), String> {
    let create_user_table = "
        CREATE TABLE IF NOT EXISTS user (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            email TEXT UNIQUE NOT NULL,
            role INTEGER NOT NULL CHECK(role IN (1, 2, 3))
        );
    ";

    let create_auth_table = "
        CREATE TABLE IF NOT EXISTS auth (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            password_hash TEXT NOT NULL,
            FOREIGN KEY(user_id) REFERENCES user(id) ON DELETE CASCADE
        );
    ";

    let mut transaction = pool.begin().await.map_err(|e| e.to_string())?;

    // Ejecutar las sentencias de creación de tablas
    sqlx::query(create_user_table)
        .execute(&mut transaction)
        .await
        .map_err(|e| e.to_string())?;

    sqlx::query(create_auth_table)
        .execute(&mut transaction)
        .await
        .map_err(|e| e.to_string())?;

    // Confirmar la transacción
    transaction.commit().await.map_err(|e| e.to_string())?;

    Ok(())
}
