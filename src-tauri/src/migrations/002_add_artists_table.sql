-- =========================
-- Tabla de artistas
-- =========================
CREATE TABLE IF NOT EXISTS artists (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    image_path TEXT,
    bio TEXT,
    profile_id INTEGER NOT NULL DEFAULT 1,
    FOREIGN KEY(profile_id) REFERENCES profiles(id) ON DELETE CASCADE
);

-- =========================
-- Índice para búsquedas rápidas
-- =========================
CREATE INDEX IF NOT EXISTS idx_artists_name ON artists(name);

-- =========================
-- Poblar tabla de artistas con los existentes
-- =========================
INSERT OR IGNORE INTO artists (name, profile_id)
SELECT DISTINCT artist, profile_id FROM songs WHERE artist IS NOT NULL;
