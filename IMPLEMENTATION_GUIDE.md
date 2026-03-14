# Guía Maestra: MusicApp Evolution

Esta guía detalla la implementación paso a paso para transformar el reproductor en una aplicación robusta con soporte multi-perfil, reproducción real basada en audio de alta calidad y gestión avanzada de bibliotecas.

---

## FASE 1: Sistema de Perfiles (Backend & Base de Datos)

**Objetivo:** Permitir que usuarios distintos ("Default", "Games", "Relax") tengan bibliotecas separadas sin duplicar archivos ni requerir autenticación compleja.

### 1.1 Nueva Migración SQL

Crear archivo: `src-tauri/src/migrations/002_profiles.sql`

```sql
-- Tabla de perfiles
CREATE TABLE IF NOT EXISTS profiles (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    avatar TEXT, -- Ruta a imagen de avatar o icono
    created_at INTEGER
);

-- Insertar perfil Default automáticamente si no existe
INSERT OR IGNORE INTO profiles (name, created_at) VALUES ('Default', strftime('%s','now'));

-- Actualizar tabla de carpetas para pertenecer a un perfil
ALTER TABLE music_folders ADD COLUMN profile_id INTEGER DEFAULT 1 REFERENCES profiles(id);

-- Actualizar tabla de canciones para filtrar por perfil
ALTER TABLE songs ADD COLUMN profile_id INTEGER DEFAULT 1 REFERENCES profiles(id);

-- Índices para mejorar rendimiento de filtros
CREATE INDEX IF NOT EXISTS idx_songs_profile ON songs(profile_id);
CREATE INDEX IF NOT EXISTS idx_folders_profile ON music_folders(profile_id);
```

### 1.2 Backend (Rust) - Gestión de Perfiles

Actualizar `src-tauri/src/db.rs` y `src-tauri/src/scan.rs`.

**Lógica de Negocio:**

1.  **Contexto de Perfil:** Todas las consultas de lectura (`SELECT`) deben incluir `WHERE profile_id = ?`.
2.  **Escaneo:** Al escanear una carpeta, el frontend debe enviar el ID del perfil activo. Las canciones insertadas deben llevar este ID.

**Nuevos Comandos (Tauri Commands):**

- `create_profile(name: String) -> Result<i32, String>`
- `get_profiles() -> Result<Vec<Profile>, String>`
- `switch_profile(profile_id: i32) -> Result<(), String>` (O simplemente manejar el ID en el frontend).

**Modificación de Comandos Existentes:**

- `add_music_folder(path: String, profile_id: i32)`
- `get_tracks(limit: i64, offset: i64, profile_id: i32)`
- `scan_folder(path: String, profile_id: i32)` -> Ahora debe pasar este ID al guardar.

---

## FASE 2: El Reproductor Real (Frontend)

**Objetivo:** Implementar un motor de audio real utilizando la **HTML5 Web Audio API** para control preciso y baja latencia, gestionado centralmente desde Pinia.

### 2.1 Store de Pinia (`src/store/player.js`)

Reemplazar la lógica simulada con una instancia real de `Audio`.

**Estructura del Store:**

```javascript
import { defineStore } from "pinia";
import { ref } from "vue";
import { convertFileSrc } from "@tauri-apps/api/core";

export const usePlayerStore = defineStore("player", () => {
  // Estado
  const audio = new Audio(); // Instancia nativa
  const isPlaying = ref(false);
  const currentTrack = ref(null);
  const queue = ref([]); // Cola de reproducción
  const queueIndex = ref(-1); // Índice actual en la cola
  const duration = ref(0);
  const currentTime = ref(0);
  const volume = ref(1);

  // Event Listeners (Configurar una vez)
  audio.ontimeupdate = () => {
    currentTime.value = audio.currentTime;
  };
  audio.ondurationchange = () => {
    duration.value = audio.duration;
  };
  audio.onended = () => {
    next();
  }; // Auto-avance

  // Acciones
  function play(track) {
    if (!track) return;

    // Si es una canción nueva
    if (currentTrack.value?.path !== track.path) {
      currentTrack.value = track;
      // IMPORTANTE: Convertir ruta local a URL permitida por Tauri
      audio.src = convertFileSrc(track.path);
      audio.load();
    }

    audio
      .play()
      .then(() => {
        isPlaying.value = true;
      })
      .catch((e) => console.error("Error playback:", e));
  }

  function pause() {
    audio.pause();
    isPlaying.value = false;
  }

  function togglePlay() {
    if (isPlaying.value) pause();
    else play(currentTrack.value);
  }

  function seek(seconds) {
    audio.currentTime = seconds;
  }

  function setVolume(val) {
    volume.value = val;
    audio.volume = val;
  }

  function next() {
    // Lógica para avanzar en 'queue'
    // Si queueIndex < queue.length - 1 -> play(queue[queueIndex + 1])
  }

  return {
    audio,
    currentTrack,
    isPlaying,
    currentTime,
    duration,
    play,
    pause,
    togglePlay,
    seek,
    setVolume,
  };
});
```

---

## FASE 3: Vistas y Navegación

**Objetivo:** Navegación fluida entre listados de álbumes y usuarios.

### 3.1 Vista de Álbum (`src/pages/AlbumView.vue`)

1.  **Ruta Dinámica:** Configurar en `router/index.js` (o donde definas rutas):
    ```javascript
    { path: '/album/:name', name: 'AlbumDetail', component: () => import('../pages/AlbumDetail.vue') }
    ```
2.  **Lógica:** Al entrar, obtener el nombre del álbum desde `route.params.name`.
3.  **Backend:** Crear comando `get_album_songs(album_name: String, profile_id: i32)` ordenada por track number.

### 3.2 Selección de Perfil

1.  **Pantalla de Login (Sin Pass):** Al iniciar la app, verificar `localStorage.getItem('active_profile')`.
2.  **Si es null:** Redirigir a `/profiles`. Mostrar cuadrícula con usuarios disponibles ("Default", "Games").
3.  **Al seleccionar:** Guardar ID en LocalStorage y Store, redirigir a `/` (Home).

---

## FASE 4: Funciones "Premium" Faltantes

**Objetivo:** Añadir las características que distinguen un reproductor básico de uno profesional.

### 4.1 Ecualizador (Frontend)

No cargar el Backend con esto. Usar la Web Audio API.

1.  Crear `AudioContext`.
2.  Crear nodo `MediaElementSource` desde el elemento `<audio>`.
3.  Conectar a `BiquadFilterNode` (Bajos, Medios, Agudos).
4.  Conectar al destino (altavoces).
5.  Controlar la ganancia (gain) de cada filtro desde deslizadores en la UI.

### 4.2 Cola de Reproducción (Queue & History)

- **Up Next:** Ver qué canción sigue.
- **Historial:** Qué sonó antes (botón Back real).
- **Shuffle Mejorado:** No solo `Math.random()`. Crear un array de índices barajados para poder ir atrás y adelante consistentemente.

### 4.3 Metadatos Avanzados

- **Lyrics (.lrc):** Buscar archivo con mismo nombre que la canción pero extensión `.lrc`. Leer contenido y parsear tiempos `[mm:ss.xx]`.
- **Track Number / Disk Number:** Necesario para ordenar álbumes correctamente (Scan debe leer estos tags).

---

## Recomendación de Orden de Trabajo

1.  **Infraestructura (Prioridad Alta):** Implementar la Fase 1 (SQL + Rust Profile Logic). Sin esto, el resto no tiene base.
2.  **Core Player (Prioridad Alta):** Implementar el Store de Pinia real (Fase 2). Sin audio real no hay app.
3.  **UI/UX:** Vistas de Album y Perfiles.
4.  **Extras:** Ecualizador y Lyrics.
