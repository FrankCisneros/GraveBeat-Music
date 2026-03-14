use sea_orm::entity::prelude::*;

pub mod song {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "songs")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        #[sea_orm(unique)]
        pub path: String,
        pub title: Option<String>,
        pub artist: Option<String>,
        pub album: Option<String>,
        pub duration: Option<i32>,
        pub cover: Option<String>,
        pub created_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::playlist_song::Entity")]
        PlaylistSong,
    }

    impl Related<super::playlist_song::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::PlaylistSong.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod playlist {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "playlists")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        pub name: String,
        pub description: Option<String>,
        pub created_at: DateTimeUtc,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(has_many = "super::playlist_song::Entity")]
        PlaylistSong,
    }

    impl Related<super::playlist_song::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::PlaylistSong.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}

pub mod playlist_song {
    use super::*;

    #[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
    #[sea_orm(table_name = "playlist_songs")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = true)]
        pub id: i32,
        pub playlist_id: i32,
        pub song_id: i32,
        pub order: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {
        #[sea_orm(
            belongs_to = "super::playlist::Entity",
            from = "Column::PlaylistId",
            to = "super::playlist::Column::Id"
        )]
        Playlist,
        #[sea_orm(
            belongs_to = "super::song::Entity",
            from = "Column::SongId",
            to = "super::song::Column::Id"
        )]
        Song,
    }

    impl Related<super::playlist::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Playlist.def()
        }
    }

    impl Related<super::song::Entity> for Entity {
        fn to() -> RelationDef {
            Relation::Song.def()
        }
    }

    impl ActiveModelBehavior for ActiveModel {}
}
