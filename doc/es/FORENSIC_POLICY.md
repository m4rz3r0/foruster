# Política de operación forense

Foruster se utiliza en escenarios de **triaje forense digital** y **análisis en vivo** sobre **sistemas en uso**. No sustituye el peritaje exhaustivo en laboratorio ni los flujos *post-mortem*; los complementa. Quien opere la herramienta debe delimitar con precisión **dónde** se lee y **dónde** se escribe, de forma que se preserve la **cadena de custodia** y se limite el impacto sobre la **evidencia digital** y el sistema anfitrión. Este documento fija el comportamiento del producto y relaciona el acceso a disco con niveles de riesgo.

**Idioma:** [English →](../en/FORENSIC_POLICY.md)

## Definiciones

| Término | Significado |
|---------|-------------|
| **Volumen analizado** | Toda ruta de sistema de archivos, disco o partición incorporada al análisis como objeto de examen (**evidencia digital**). |
| **Árbol portátil** | Directorio que contiene el binario de Foruster y, opcionalmente, `data/` (kit USB, recurso compartido preparado para el caso, etc.). Se fija con `FORUSTER_PORTABLE_ROOT` si el diseño difiere de «binario junto a `data/`». |
| **Perfil del SO anfitrión** | Configuración de usuario del sistema operativo (p. ej. `%APPDATA%`, `~/.config`) — **no** forma parte del kit portátil. |
| **Exportación dirigida por el usuario** | Guardar solo mediante un diálogo **Guardar** / **Exportar** donde el investigador elige el destino. |

## Matriz de política

| Ámbito | Objetivo |
|--------|----------|
| **Volúmenes analizados** | Solo lectura desde la perspectiva de Foruster: no crear, modificar ni eliminar archivos de evidencia. El análisis usa recorrido de directorios y lectura de archivos; los plugins reciben únicamente manejadores registrados. |
| **Disco del sistema anfitrión** | Minimizar escrituras silenciosas. Preferir el **árbol portátil** para configuración, modelos, conjuntos de hashes y datos temporales cuando el **modo forense** está activo. |
| **Exportaciones (informes, CSV, JSON, imágenes)** | Permitidas solo como **exportaciones dirigidas por el usuario** a rutas elegidas en la interfaz — nunca escrituras silenciosas a ubicaciones arbitrarias. |

## Comportamiento predeterminado (sin configuración adicional)

El **modo forense está activo por defecto** al iniciar la aplicación (doble clic o línea de comandos sin opciones). No se exigen variables de entorno para auditorías legales o forenses: preferencias y datos temporales van bajo el árbol portátil `data/` junto al binario (o bajo `FORUSTER_PORTABLE_ROOT`), no bajo el perfil de usuario del anfitrión ni el temporal del sistema.

Para usar el diseño **estándar** (perfil de usuario del SO, directorio temporal del sistema, posible siembra automática de conjuntos de hashes si no existe configuración), inicie con:

- **`--standard`** o **`--no-forensic`** en la línea de comandos, o  
- **`FORUSTER_FORENSIC_MODE=0`** (o `false` / `no`), o  
- **`FORUSTER_STANDARD_MODE=1`** solo cuando `FORUSTER_FORENSIC_MODE` no tenga un valor no vacío.

Ejecute **`foruster-desktop --help`** para un resumen breve (desde terminal; en Windows, las compilaciones GUI pueden no mostrar consola al abrir desde el Explorador).

**Precedencia:** la línea de comandos `--forensic` / `--standard` / `--no-forensic` prevalece sobre `FORUSTER_FORENSIC_MODE` y `FORUSTER_STANDARD_MODE` cuando está presente.

## Variables de entorno

| Variable | Efecto |
|----------|--------|
| `FORUSTER_PORTABLE_ROOT` | Ruta absoluta a la raíz del despliegue portátil (sustituye el «directorio del ejecutable actual»). El `data/` escribible queda bajo esta raíz. |
| `FORUSTER_FORENSIC_MODE` | Si **no está definida** o está **vacía**, el modo forense está **activo** salvo que `FORUSTER_STANDARD_MODE` indique lo contrario (véase arriba). Si tiene valor no vacío: `0` / `false` / `no` → modo estándar; cualquier otro valor → modo forense. |
| `FORUSTER_STANDARD_MODE` | Con valor verdadero y `FORUSTER_FORENSIC_MODE` no definida o vacía → modo estándar. Se ignora cuando `FORUSTER_FORENSIC_MODE` está explícitamente no vacía. |
| `FORUSTER_NO_DEFAULT_HASHSETS` | Si está definida, omite por completo la carga y siembra portátil de conjuntos de hashes (opción heredada de exclusión). |

## Inventario de operaciones de entrada/salida (E/S)

Las operaciones son **lectura (R)** o **escritura (W)**. El «riesgo» es el impacto en el **sistema analizado** si la aplicación se ejecuta desde o se instala en ese sistema sin kit portátil.

| Componente / operación | Volumen objetivo | R/W | Riesgo | Mitigación |
|------------------------|------------------|-----|--------|------------|
| Recorredor de análisis / hashing | Rutas analizadas | R | Bajo si se usan APIs de solo lectura | Linux: `O_NOATIME` cuando proceda; véase [acceso a archivos en Windows](#acceso-a-archivos-y-marcas-de-tiempo-en-windows). |
| Host de plugin: `read_file`, `compute_hash`, `decode_image`, `run_inference` | Rutas pasadas como `FileEntry` registrado | R | Bajo | Límites de lectura por invocación; sandbox WASM (sin rutas arbitrarias). |
| Host de plugin: `query_sqlite` | Copia de la BD en temporal | W (solo temporal) | Medio si el temporal es del SO | Modo forense: copias bajo `data/scratch/` portátil. |
| Escritorio: preferencias de interfaz | Perfil del anfitrión **o** `data/config/` | W | Medio (perfil anfitrión) | Modo forense: solo árbol portátil. |
| Escritorio: conjuntos de hashes portátiles | `data/hashsets/` | R/W | Bajo si el kit está en medio extraíble | Modo forense: sin siembra automática; opcional `FORUSTER_NO_DEFAULT_HASHSETS`. |
| Escritorio: exportación PDF / JSON / CSV / imagen | Ruta elegida por el usuario | W | Controlado por el usuario | Solo diálogos de guardado. |
| Escritorio: «Abrir carpeta» tras exportar | El SO abre el explorador | Indirecto | Entorno del usuario | El investigador elige la ruta de exportación. |
| Ayudante de cookies del navegador | Copia la BD al temporal y lanza el navegador | W (temporal) | Medio | Modo forense: temporal bajo árbol portátil. |
| Instalador / `foruster-installer` | Rutas de descarga / desempaquetado | W | **Herramienta de preparación** | No pensado para ejecutarse en la máquina analizada durante la adquisición; use otro sistema para construir el kit. |

## Acceso a archivos y marcas de tiempo en Windows

- **Linux** usa `O_NOATIME` al abrir archivos para detección de formato cuando el núcleo y los permisos lo permiten, con recurso a apertura normal si `EPERM`.
- **Windows** no expone un equivalente directo a `O_NOATIME` en la biblioteca estándar. El comportamiento de último acceso depende de la **política NTFS** (p. ej. las actualizaciones de último acceso pueden desactivarse en todo el sistema por rendimiento). Abrir archivos para lectura puede actualizar metadatos según la configuración del SO.
- **Recomendación:** ejecute la herramienta desde un **medio portátil** con `FORUSTER_FORENSIC_MODE` activo, documente la **versión del SO anfitrión** y el **sistema de archivos**, y considere la preservación de marcas de tiempo como **mejor esfuerzo** en Windows salvo controladores especializados o bloqueadores de escritura a nivel de volumen.

## Documentación relacionada

- [STATIC_BUILDS.md](STATIC_BUILDS.md) — compilaciones estáticas / musl y dependencias en tiempo de ejecución.
- [HASH_SETS.md](HASH_SETS.md) — disposición de conjuntos de hashes bajo `data/hashsets/`.
- [PLUGIN_SDK.md](PLUGIN_SDK.md) — límites de E/S del host para plugins.
- [INDEX.md](INDEX.md) — índice de documentación de plugins e instalación.
