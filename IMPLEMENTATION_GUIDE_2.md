# Plan de Implementación Fase 2: Optimización y Nuevas Funcionalidades

Este documento detalla la hoja de ruta para la segunda fase de desarrollo de MusicApp. El enfoque principal es la optimización del rendimiento, la integración de funcionalidades nativas (atajos, bandeja del sistema) y mejoras significativas en la experiencia de usuario (letras, vista detallada).

---

## 1. Vista de Detalles de Canción "Now Playing"
Crear una vista inmersiva para cuando el usuario quiere enfocarse en la música.

*   **Tipo**: Frontend (`Vue`)
*   **Ubicación**: Nueva ruta `/player` o un overlay a pantalla completa.
*   **Características**:
    *   Carátula del álbum en alta resolución.
    *   Visualización de letras (Lyrics) sincronizadas o estáticas.
    *   Fondo dinámico basado en los colores de la carátula (usando bibliotecas como `colorthief` o extracción de CSS).
*   **Pasos**:
    1.  Crear componente `NowPlaying.vue`.
    2.  Implementar botón en la barra de reproducción para expandir esta vista.
    3.  Añadir transiciones suaves de entrada/salida.

---

## 2. Integración de Letras (Lyrics)
Mostrar la letra de la canción actual.

*   **Tipo**: Híbrido (Backend para Proxy/Cache + Frontend para visualización)
*   **API Recomendada**: **Lrclib.net** (Gratuita, Open Source, buena API). O usar `genius-lyrics` (scraper, puede ser más lento).
*   **Pasos**:
    1.  **Backend (Rust/Tauri)**:
        *   Crear comando `get_lyrics(title, artist)`.
        *   (Opcional) Crear tabla `lyrics` en la DB para cachear resultados y no pedir siempre a internet.
    2.  **Frontend**:
        *   Llamar a la API cuando cambie la canción (`watch` currentTrack).
        *   Si hay datos de tiempo (LRC), implementar scroll automático sincronizado con `audio.currentTime`.

---

## 3. Corrección de Metadatos de Álbum (Agrupación)
Solucionar el problema de álbumes divididos por estar en carpetas diferentes (ej: `Music/Games/Bloodborne` vs `Music/OST/Bloodborne`).

*   **Tipo**: Backend (Rust/SQL) y Frontend
*   **Problema**: Actualmente la agrupación probablemente considera la ruta o el sistema de archivos demasiado estricto.
*   **Solución**:
    1.  **Backend**: Modificar la consulta SQL de `get_albums` (o la lógica en JS si se hace ahí) para agrupar estrictamente por `Album Name` y `Artist` (o `Album Artist`), ignorando la ruta del archivo (`path`).
    2.  **Identificación**: Asegurarse de normalizar nombres (trim espacios, ignorar mayúsculas/minúsculas) al agrupar.
    3.  **Edición (Futuro)**: Crear una función para "Fusionar álbumes" manualmente si los nombres difieren ligeramente.

---

## 4. Mejoras en Sidebar y Playlists
Mostrar y gestionar playlists creadas en el perfil actual.

*   **Tipo**: Fullstack
*   **Pasos**:
    1.  **Backend (DB)**:
        *   Crear tablas: `playlists` (id, name, profile_id) y `playlist_songs` (playlist_id, song_path/id, order).
        *   Crear comandos Rust: `create_playlist`, `add_to_playlist`, `get_playlists`.
    2.  **Frontend (Sidebar)**:
        *   Añadir sección "Mis Playlists".
        *   Listar las playlists traídas de la DB.
        *   Permitir crear nueva playlist (botón +).
        *   Drag & Drop de canciones a la playlist en la sidebar (Avanzado).

---

## 5. Optimización de Rendimiento (App Trabada)
Este es un punto crítico. La UI no debe congelarse durante tareas pesadas.

*   **Problema 1: Escaneo bloqueante**
    *   **Causa**: Probablemente el escaneo de archivos y la insersión en DB se hacen en el hilo principal o bloquean el runtime de Tokio de forma síncrona, o se están emitiendo demasiados eventos al frontend.
    *   **Solución (Rust)**:
        *   Ejecutar el escaneo en un hilo separado (`std::thread::spawn` o `tokio::spawn_blocking`).
        *   **Batch Insert**: No insertar canciones 1 por 1 en la SQL. Insertar en lotes (chunks) de 50 o 100 canciones dentro de una sola transacción. Esto acelera la DB x100.
        *   Reducir eventos: No enviar `emit('scan-progress')` por cada archivo. Enviarlo cada 50 archivos o cada 1 segundo.

*   **Problema 2: Lag con muchas canciones (Renderizado)**
    *   **Causa**: Renderizar una lista de DOM con 2000+ elementos (`v-for`) mata al navegador.
    *   **Solución (Frontend)**:
        *   Implementar **Virtual Scrolling**. Usar librería `vue-virtual-scroller` o `@tanstack/vue-virtual`.
        *   Esto solo renderiza lo que se ve en pantalla + un pequeño buffer. Permite listas de 100,000 canciones sin lag.

---

## 6. Atajos de Teclado Globales y Media Keys
Controlar la música aunque la app no tenga el foco.

*   **Tipo**: Backend (Rust)
*   **Herramienta**: crate `tauri-plugin-global-shortcut` (para teclas custom) o integración nativa de OS Media Controls (Play, Pause, Next, Prev en el teclado).
*   **Pasos**:
    1.  Configurar plugin en `main.rs`.
    2.  Escuchar teclas `MediaPlayPause`, `MediaNextTrack`, `MediaPrevTrack`.
    3.  Emitir evento al frontend `window.emit('media-play-pause')` para que `player.js` actúe.

---

## 7. Mini Reproductor y Bandeja del Sistema (Tray)
Minimizar a la bandeja con opciones rápidas.

*   **Tipo**: Backend (Rust) + Frontend (Opcional ventana)
*   **Nivel Básico (Recomendado Primero)**:
    *   Icono en Tray (al lado del reloj).
    *   Menú clic derecho: "Reproducir/Pausar", "Siguiente", "Salir".
    *   Al minimizar la ventana principal, ocultarla (`hide()`) en lugar de cerrar, y mantener el Tray.
*   **Nivel Avanzado (Mini Ventana)**:
    *   Crear una segunda ventana en Tauri (`label: "mini-player"`).
    *   Diseño pequeño (tipo widget).
    *   Hacer que aparezca al hacer clic izquierdo en el icono del Tray.

---

## 8. Mejoras Visuales y de Interfaz (UI/UX)
Recomendaciones para un look & feel "Premium".

*   **Frontend**:
    1.  **Glassmorphism**: Usar fondos con transparencia y `backdrop-filter: blur(20px)` en la Sidebar y Player Controls para que el contenido pase por detrás sutilmente.
    2.  **Transiciones**: Usar `<Transition>` de Vue para cambios de página (fade, slide).
    3.  **Feedback**: Efectos "Ripple" al hacer clic en botones.
    4.  **Skeleton Loading**: Mientras cargan los álbumes (si tarda), mostrar esqueletos grises en lugar de un espacio en blanco.
    5.  **Scrollbars**: Personalizar la barra de scroll (`::-webkit-scrollbar`) para que sea fina y del color del tema. 

---

## Resumen de Prioridades
1.  **Optimización (Punto 5)**: Es urgente para que la app sea usable con bibliotecas grandes.
2.  **DB & Agrupación (Punto 3)**: Esencial para tener la biblioteca ordenada.
3.  **Atajos (Punto 6)**: Funcionalidad básica esperada en desktop.
4.  **Lyrics & Now Playing (Puntos 1 y 2)**: Valor añadido visual.
