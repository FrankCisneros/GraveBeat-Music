# GraveBeat 🎶

**GraveBeat** es un reproductor de música local de alto rendimiento y diseño premium. Está construido sobre un potente backend en **Rust** (con **Tauri**) y una interfaz de usuario moderna y fluida desarrollada con **Vue 3**, **Tailwind CSS** y **DaisyUI**. 

El objetivo principal de GraveBeat es ofrecer una experiencia inmersiva para escanear, organizar y reproducir tu biblioteca de música local de forma eficiente, permitiendo una personalización completa y aislamiento a través de perfiles independientes.

---

## 🚀 Características Clave (Cómo está implementado)

### 🖥️ Arquitectura e Interfaz
*   **Backend Híbrido (Rust + Tauri v2):** Comunicación rápida y segura entre el frontend y el backend nativo. Uso de plugins nativos para sistema de archivos, diálogos del sistema y atajos globales.
*   **Frontend Moderno (Vue 3 + Pinia):** Interfaz reactiva y rápida con un sistema de estados centralizado para la reproducción y configuración.
*   **Diseño Premium (Tailwind CSS + DaisyUI):** UI elegante con soporte para múltiples temas visuales (oscuro, claro y variaciones personalizadas).

### 📂 Escaneo y Base de Datos
*   **Escaneo Inteligente con Lofty (Rust):** Escanea archivos locales (`.mp3`, `.flac`, `.wav`, `.ogg`, `.m4a`), extrayendo metadatos como título, artista, álbum y duración.
*   **Caché de Carátulas (MD5):** Extrae automáticamente la carátula integrada en las canciones, la guarda en la caché del sistema utilizando un hash MD5 único por álbum para evitar duplicados y optimizar la carga.
*   **Base de Datos SQLite (sqlx):** Estructura relacional con migraciones automáticas (`sqlx::migrate`) que almacena pistas, carpetas, favoritos, perfiles y listas de reproducción.
*   **Detección de Cambios:** Comprobación rápida (`is_folder_modified`) para determinar si una carpeta ha cambiado desde el último escaneo y evitar re-escaneos innecesarios.

### 🎧 Audio y Reproducción
*   **Motor de Audio Avanzado (Howler.js & Web Audio API):** Soporte completo para reproducción continua, control de volumen con límites de seguridad y cola de reproducción persistente (`player_playlist`).
*   **Ecualizador Gráfico de 20 Bandas:** Ecualizador integrado con múltiples presets predefinidos (*Rock, Pop, Jazz, Classical, Bass, Dance, Acoustic*, etc.) y control manual (*Custom*).
*   **Atajos de Teclado y Teclas Multimedia:** Integración con atajos de teclado locales (Barra espaciadora para reproducir/pausar) y teclas multimedia globales (`MediaPlayPause`, `MediaTrackNext`, `MediaTrackPrevious`) gracias a `tauri-plugin-global-shortcut`.

### 👥 Perfiles y Personalización
*   **Aislamiento de Perfiles:** Permite crear múltiples perfiles de usuario. Cada perfil gestiona de forma independiente sus carpetas de música, favoritos, listas de reproducción y preferencias sin interferir entre sí.
*   **Integración con Discord (Rich Presence):** Muestra de forma dinámica en tu perfil de Discord lo que estás escuchando en tiempo real (Título, Artista, Duración y estado de reproducción).
*   **Gestión de Playlists y Favoritos:** Agrega canciones a favoritos y crea listas de reproducción personalizadas desde menús contextuales.

---

## 📊 Estado del Proyecto

*   **Progreso Actual:** **90%** 🟩🟩🟩🟩🟩🟩🟩🟩🟩⬜ (Casi finalizado)
*   La aplicación cuenta con toda la infraestructura central de base de datos, reproducción de audio nativo de alta calidad, ecualización, perfiles independientes y escaneo en Rust finalizados y estables.

---

## 📝 Cosas que faltan (Pendientes)

A continuación se listan las características y detalles pendientes para completar el proyecto al 100%:

- [ ] **Optimización de metadatos:** Corregir y pulir la lectura de ciertos metadatos avanzados (como número de pista/disco) para un mejor ordenamiento de álbumes.
- [ ] **Vista inmersiva de reproducción ("Now Playing"):** Desarrollar una interfaz detallada a pantalla completa para visualizar la letra (Lyrics sincronizadas a través de API como Lrclib) y la carátula en alta definición.
- [ ] **Compartir álbumes entre perfiles:** Permitir compartir la visualización de ciertos álbumes entre distintos perfiles de usuario sin mezclar las carpetas físicas escaneadas.
- [ ] **Corrección de detalles en perfiles:** Solucionar pequeños fallos visuales y de refresco al cambiar de perfil de usuario en caliente.
- [ ] **Personalización visual adicional:** Agregar más opciones para temas personalizados y retoques en la interfaz.
- [ ] **Integración en la bandeja del sistema (System Tray / Mini reproductor):** Soporte para minimizar al tray y controlar la app mediante un widget flotante.
- [ ] **Virtual Scrolling:** Implementar virtualización de listas para bibliotecas extremadamente grandes (+5,000 canciones) para garantizar 0 lag.