use anyhow::Result;
use sqlx::SqlitePool;

pub async fn initialize(pool: &SqlitePool) -> Result<()> {
    initialize_sets(pool).await?;
    initialize_parts(pool).await?;
    initialize_part_variants(pool).await?;
    initialize_colors(pool).await?;
    initialize_set_parts(pool).await?;
    initialize_user_sets(pool).await?;
    initialize_user_part_progress(pool).await?;

    Ok(())
}



async fn initialize_sets(pool: &SqlitePool) -> Result<()> {
    sqlx::query(r#"
    CREATE TABLE IF NOT EXISTS sets (
        set_num TEXT PRIMARY KEY,

        name TEXT NOT NULL,
        year INTEGER NOT NULL,
        theme_id INTEGER NOT NULL,
        num_parts INTEGER NOT NULL,

        remote_image_url TEXT,
        local_image_path TEXT,

        set_url TEXT,
        last_modified TEXT
    );
    "#,).execute(pool).await?;

    Ok(())
}


async fn initialize_parts(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS parts (
        part_num TEXT PRIMARY KEY,
        name TEXT NOT NULL,
        category_id INTEGER NOT NULL
    );
    "#,
    ).execute(pool).await?;

    Ok(())
}

async fn initialize_colors(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS colors (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        rgb TEXT NOT NULL,
        is_trans INTEGER NOT NULL
    );
    "#
    ).execute(pool).await?;

    Ok(())
}

async fn initialize_part_variants(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS part_variants (

        element_id TEXT PRIMARY KEY,

        part_num TEXT NOT NULL,

        color_id INTEGER NOT NULL,

        remote_image_url TEXT,

        local_image_path TEXT,

        FOREIGN KEY(part_num)
            REFERENCES parts(part_num),

        FOREIGN KEY(color_id)
            REFERENCES colors(id)
    );
    CREATE INDEX IF NOT EXISTS idx_part_variants
    ON part_variants(part_num, color_id);
    "#
    ).execute(pool).await?;

    Ok(())
}

async fn initialize_set_parts(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS set_parts (

        set_num TEXT NOT NULL,

        element_id TEXT NOT NULL,

        quantity INTEGER NOT NULL,

        is_spare INTEGER NOT NULL,

        PRIMARY KEY(set_num, element_id, is_spare),

        FOREIGN KEY(set_num)
            REFERENCES sets(set_num),

        FOREIGN KEY(element_id)
            REFERENCES part_variants(element_id)
    );
    CREATE INDEX IF NOT EXISTS idx_set_parts_set
    ON set_parts(set_num);
    "#,
    ).execute(pool).await?;

    Ok(())
}

async fn initialize_user_sets(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS user_sets (

        set_num TEXT PRIMARY KEY,

        status TEXT NOT NULL,

        added_at TEXT NOT NULL,

        started_at TEXT,

        completed_at TEXT,

        FOREIGN KEY(set_num)
            REFERENCES sets(set_num)
    );
    CREATE INDEX IF NOT EXISTS idx_user_sets_status
    ON user_sets(status);
    "#,
    ).execute(pool).await?;

    Ok(())
}

async fn initialize_user_part_progress(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
    CREATE TABLE IF NOT EXISTS user_part_progress (

        set_num TEXT NOT NULL,

        element_id TEXT NOT NULL,

        built_quantity INTEGER NOT NULL DEFAULT 0,

        is_spare INTEGER NOT NULL,

        PRIMARY KEY(
            set_num,
            element_id,
            is_spare
        ),

        FOREIGN KEY(set_num)
            REFERENCES user_sets(set_num),

        FOREIGN KEY(element_id)
            REFERENCES part_variants(element_id)
    );
    CREATE INDEX IF NOT EXISTS idx_progress_element
    ON user_part_progress(element_id);
    "#,
    ).execute(pool).await?;

    Ok(())
}


pub async fn cleanup(pool: &SqlitePool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DELETE FROM user_part_progress;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM user_sets;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM set_parts;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM part_variants;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM parts;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM colors;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DELETE FROM sets;")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn drop_tables(pool: &SqlitePool) -> Result<()> {
    let mut tx = pool.begin().await?;

    sqlx::query("DROP TABLE IF EXISTS user_part_progress;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS user_sets;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS set_parts;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS part_variants;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS parts;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS colors;")
        .execute(&mut *tx)
        .await?;

    sqlx::query("DROP TABLE IF EXISTS sets;")
        .execute(&mut *tx)
        .await?;

    tx.commit().await?;

    Ok(())
}

pub async fn reset(pool: &SqlitePool) -> Result<()> {
    cleanup(pool).await?;
    initialize(pool).await?;
    Ok(())
}

pub async fn recreate(pool: &SqlitePool) -> Result<()> {
    drop_tables(pool).await?;
    initialize(pool).await?;
    Ok(())
}