use crate::entities::{playlist, playlist_song, song};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};

// Connect to the database or create it if it doesn't exist
pub async fn connect() -> DatabaseConnection {
    Database::connect("sqlite://music.db?mode=rwc")
        .await
        .expect("Error conectando a SQLite")
}

pub async fn create_tables(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);

    let mut song_stmt = schema.create_table_from_entity(song::Entity);
    song_stmt.if_not_exists();
    let _ = db.execute(&song_stmt).await;

    let mut playlist_stmt = schema.create_table_from_entity(playlist::Entity);
    playlist_stmt.if_not_exists();
    let _ = db.execute(&playlist_stmt).await;

    let mut ps_stmt = schema.create_table_from_entity(playlist_song::Entity);
    ps_stmt.if_not_exists();
    let _ = db.execute(&ps_stmt).await;
}
