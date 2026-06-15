# Política de operación forense

Foruster se utiliza en escenarios de **triaje forense digital** y **análisis en vivo** sobre **sistemas en uso**. No sustituye el peritaje exhaustivo en laboratorio ni el análisis centrado en una imagen forense adquirida con anterioridad; lo complementa. Quien opere la herramienta debe delimitar con precisión **dónde** se lee y **dónde** se escribe, de forma que se preserve la **cadena de custodia** y se limite el impacto sobre la **evidencia digital** y el **equipo donde se ejecuta el examen**. Este documento fija el comportamiento del producto y relaciona el acceso a disco con niveles de riesgo.

## Definiciones

| Término | Significado |
|---------|-------------|
| **Volumen analizado** | Toda ruta de sistema de archivos, disco o partición incorporada al análisis como objeto de examen (**evidencia digital**). |
| **Kit portátil** | Carpeta que contiene el ejecutable de Foruster y, cuando aplica, el directorio `data/` (por ejemplo un USB o un recurso compartido preparado para el caso). Si la disposición no es «ejecutable junto a `data/`», fíjela con `FORUSTER_PORTABLE_ROOT`. |
| **Perfil de usuario del sistema operativo** | Configuración de usuario del sistema operativo del equipo en examen (p. ej. `%APPDATA%`, `~/.config`) — **no** forma parte del kit portátil. |
| **Exportación dirigida por el usuario** | Guardar solo mediante un diálogo **Guardar** / **Exportar** donde el investigador elige el destino. |

## Matriz de política

| Ámbito | Objetivo |
|--------|----------|
| **Volúmenes analizados** | Solo lectura desde la perspectiva de Foruster: no crear, modificar ni eliminar archivos de evidencia. El análisis usa recorrido de directorios y lectura de archivos; los plugins reciben únicamente manejadores registrados. |
| **Disco del sistema en examen** | Minimizar escrituras silenciosas. Preferir el **kit portátil** para configuración, modelos, conjuntos de hashes y datos temporales cuando el **modo forense** está activo. |
| **Exportaciones (informes, CSV, JSON, imágenes)** | Permitidas solo como **exportaciones dirigidas por el usuario** a rutas elegidas en la interfaz — nunca escrituras silenciosas a ubicaciones arbitrarias. |

## Comportamiento predeterminado (sin configuración adicional)

El **modo forense está activo por defecto** al iniciar la aplicación (doble clic o línea de comandos sin opciones). No se exigen variables de entorno para auditorías legales o forenses: preferencias y datos temporales van bajo `data/` **dentro del kit portátil** junto al binario (o bajo `FORUSTER_PORTABLE_ROOT`), no bajo el perfil de usuario del sistema ni el directorio temporal del sistema operativo.

Para usar el diseño **estándar** (perfil de usuario del SO, directorio temporal del sistema, posible siembra automática de conjuntos de hashes si no existe configuración), inicie con:

- **`--standard`** o **`--no-forensic`** en la línea de comandos, o  
- **`FORUSTER_FORENSIC_MODE=0`** (o `false` / `no`), o  
- **`FORUSTER_STANDARD_MODE=1`** solo cuando `FORUSTER_FORENSIC_MODE` no tenga un valor no vacío.

Ejecute **`foruster-desktop --help`** para un resumen breve (desde terminal; en Windows, las compilaciones GUI pueden no mostrar consola al abrir desde el Explorador).

**Precedencia:** la línea de comandos `--forensic` / `--standard` / `--no-forensic` prevalece sobre `FORUSTER_FORENSIC_MODE` y `FORUSTER_STANDARD_MODE` cuando está presente.

## Variables de entorno

| Variable | Efecto |
|----------|--------|
| `FORUSTER_PORTABLE_ROOT` | Ruta absoluta a la raíz del kit portátil (sustituye el directorio del ejecutable). El directorio `data/` escribible queda bajo esa raíz. |
| `FORUSTER_FORENSIC_MODE` | Si **no está definida** o está **vacía**, el modo forense está **activo** salvo que `FORUSTER_STANDARD_MODE` indique lo contrario (véase arriba). Si tiene valor no vacío: `0` / `false` / `no` → modo estándar; cualquier otro valor → modo forense. |
| `FORUSTER_STANDARD_MODE` | Con valor verdadero y `FORUSTER_FORENSIC_MODE` no definida o vacía → modo estándar. Se ignora cuando `FORUSTER_FORENSIC_MODE` está explícitamente no vacía. |
| `FORUSTER_NO_DEFAULT_HASHSETS` | Si está definida, omite por completo la carga y siembra portátil de conjuntos de hashes (opción heredada de exclusión). |

## Inventario de operaciones de entrada/salida (E/S)

Las operaciones son **lectura (R)** o **escritura (W)**. El «riesgo» es el impacto en el **sistema analizado** si la aplicación se ejecuta desde o se instala en ese sistema **sin** un kit portátil preparado para el caso.

| Componente / operación | Volumen objetivo | R/W | Riesgo | Mitigación |
|------------------------|------------------|-----|--------|------------|
| Recorredor de análisis / hashing | Rutas analizadas | R | Bajo si se usan APIs de solo lectura | Linux: `O_NOATIME` cuando proceda; véase [acceso a archivos en Windows](#acceso-a-archivos-y-marcas-de-tiempo-en-windows). |
| Capa de plugin: `read_file`, `compute_hash`, `decode_image`, `run_inference` | Rutas pasadas como `FileEntry` registrado | R | Bajo | Límites de lectura por invocación; entorno de ejecución WebAssembly aislado (sin rutas arbitrarias). |
| Capa de plugin: `query_sqlite` | Copia de la BD en temporal | W (solo temporal) | Medio si el temporal es del SO | Modo forense: copias bajo `data/scratch/` del kit portátil. |
| Escritorio: preferencias de interfaz | Perfil de usuario del SO **o** `data/config/` | W | Medio (perfil de usuario del SO) | Modo forense: solo kit portátil. |
| Escritorio: conjuntos de hashes portátiles | `data/hashsets/` | R/W | Bajo si el kit está en medio extraíble | Modo forense: sin siembra automática; opcional `FORUSTER_NO_DEFAULT_HASHSETS`. |
| Escritorio: exportación PDF / JSON / CSV / imagen | Ruta elegida por el usuario | W | Controlado por el usuario | Solo diálogos de guardado. |
| Escritorio: «Abrir carpeta» tras exportar | El SO abre el explorador | Indirecto | Entorno del usuario | El investigador elige la ruta de exportación. |
| Ayudante de cookies del navegador | Copia la BD al temporal y lanza el navegador | W (temporal) | Medio | Modo forense: temporal bajo el kit portátil. |
| Instalador / `foruster-installer` | Rutas de descarga / desempaquetado | W | **Herramienta de preparación** | No pensado para ejecutarse en la máquina analizada durante la adquisición; use otro sistema para construir el kit. |

## Acceso a archivos y marcas de tiempo en Windows

- **Linux** usa `O_NOATIME` al abrir archivos para detección de formato cuando el núcleo y los permisos lo permiten, con recurso a apertura normal si `EPERM` (véase `crates/analysis/src/utils.rs`).
- **Windows** no expone un equivalente directo a `O_NOATIME` en la biblioteca estándar. El comportamiento de último acceso depende de la **política NTFS** (p. ej. las actualizaciones de último acceso pueden desactivarse en todo el sistema por rendimiento). Abrir archivos para lectura puede actualizar metadatos según la configuración del SO.
- **Recomendación:** ejecute la herramienta desde un **medio extraíble** con `FORUSTER_FORENSIC_MODE` activo, documente la **versión del sistema operativo del equipo** y el **sistema de archivos**, y considere la preservación de marcas de tiempo como **mejor esfuerzo** en Windows salvo controladores especializados o bloqueadores de escritura a nivel de volumen.

## Documentación relacionada

### Guías de operador (esta carpeta)

- [Bases de datos de huellas (NSRL y listas)](HASH_SETS.md) — conjuntos bajo `data/hashsets/`.
- [Guía de instalación y preparación del kit](INSTALLER.md) — puesto de preparación y disposición del kit.
- [Índice de temas](INDEX.md) — mapa de la documentación para operadores.

---

[Descargas e inicio de la aplicación](https://m4rz3r0.github.io/foruster/)
