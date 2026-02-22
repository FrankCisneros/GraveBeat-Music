-- =========================
-- Tabla de perfiles
-- =========================
CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    avatar_path TEXT,
    settings TEXT,
    created_at INTEGER,
    updated_at INTEGER
);

-- =========================
-- Tabla de carpetas (por perfil)
-- =========================
CREATE TABLE IF NOT EXISTS music_folders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    added_at INTEGER,
    profile_id INTEGER NOT NULL,
    UNIQUE(path, profile_id),
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

-- =========================
-- Tabla de canciones (por perfil)
-- =========================
CREATE TABLE IF NOT EXISTS songs (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    name TEXT,
    title TEXT,
    artist TEXT,
    album TEXT,
    duration INTEGER,
    cover_path TEXT,
    last_modified INTEGER,
    profile_id INTEGER NOT NULL,
    UNIQUE(path, profile_id),
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

-- =========================
-- Perfil por defecto
-- =========================
INSERT OR IGNORE INTO profiles (name, created_at, updated_at)
VALUES ('default', strftime('%s','now'), strftime('%s','now'));

-- =========================
-- Índices
-- =========================
CREATE INDEX IF NOT EXISTS idx_songs_profile ON songs(profile_id);
CREATE INDEX IF NOT EXISTS idx_folders_profile ON music_folders(profile_id);
CREATE INDEX IF NOT EXISTS idx_artist ON songs(artist);
CREATE INDEX IF NOT EXISTS idx_album ON songs(album);
CREATE INDEX IF NOT EXISTS idx_title ON songs(title);
CREATE INDEX IF NOT EXISTS idx_profile ON profiles(id);