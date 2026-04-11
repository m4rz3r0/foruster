# Instalador de Foruster (`foruster-installer`)

Utilidad gráfica para copiar la aplicación Foruster y activos opcionales en un **medio de despliegue** (por ejemplo un USB preparado en un laboratorio). Ejecútela **solo en el equipo de preparación**, no en el sistema objeto de examen forense.

**Idioma:** [English →](../en/INSTALLER.md)

## Modos

| Modo | Uso |
|------|-----|
| **Sin conexión** | El instalador está junto a un **paquete** local (`foruster` / `foruster.exe` y carpetas de soporte). No se descarga nada. |
| **En línea** | Obtiene paquetes de publicación y manifiestos de extensiones desde GitHub (TLS). Requiere red **solo en este PC de preparación**. |

Defina `FORUSTER_BUNDLE_ROOT` si el paquete no está junto al ejecutable del instalador.

## Versiones (modo en línea)

Este bloque es **siempre visible**. En modo **Sin conexión** los controles están **deshabilitados** (y la tarjeta se atenúa visualmente) para que cambiar el ORIGEN **no** desplace el resto del formulario — mejor accesibilidad y disposición predecible. Cuando el ORIGEN es **En línea**, active **Mostrar selección de versiones** para elegir:

- **Publicación de la aplicación** — Etiquetas de GitHub que publican un paquete por plataforma para el SO elegido (o **Última** para la publicación predeterminada).
- **Publicación de extensiones** — Una etiqueta `plugins/…` para `plugins-manifest.json` (o **Últimos plugins**).

Use **Aplicar** junto a la lista de extensiones para volver a cargar el manifiesto tras cambiar la etiqueta de extensiones. **Actualizar listas de versiones** vuelve a consultar a GitHub las etiquetas (p. ej. tras una nueva publicación).

## Plataforma de destino

Elija **Linux** o **Windows** para que el nombre del binario instalado coincida con el árbol de destino. Cambiar el objetivo actualiza la lista de etiquetas de la **aplicación** cuando hay conexión.

## Bases de datos de hashes (`data/hashsets/`)

El instalador escribe todo bajo la **carpeta de destino** elegida — diseño portátil: `data/hashsets/` junto a la aplicación. No se guarda nada bajo el perfil de usuario del anfitrión.

Puede configurar **tres niveles** de forma independiente:

1. **Conocidos buenos (NSRL / software catalogado)**  
   - **Ninguno** — sin SQLite de conocidos buenos en la configuración (otros niveles pueden estar definidos).  
   - **Muestra mínima** — SQLite RDSv3 incrustada pequeña (~8 KiB), funciona **sin red**.  
   - **Demo curada NIST** — descarga el zip público RDSv3 curado del NIST (~87 MiB). Requiere modo **en línea**.  
   - **Demo curada legada NIST** — segundo zip curado desde un diseño antiguo del NIST; requiere **en línea**. Si el NIST retira la URL, elija otra opción o coloque un archivo manualmente.  
   - **Publicaciones NSRL completas** — **Android minimal**, **Legacy minimal**, **Modern minimal** o **Modern complete**: zips RDSv3 del NIST (multigigabyte; la versión de publicación está fijada en el instalador). Requiere **en línea**, diálogo de **confirmación** y espacio en disco suficiente (el instalador comprueba aproximadamente **2×** el tamaño esperado de descarga). Son descargas **oficiales del NIST**, no archivos generados en su máquina.

2. **Lista de alerta nivel sospechoso**  
   - **Ninguno** — sin archivo.  
   - **Marcador vacío (.txt)** — crea `alert_suspicious.txt` solo con comentarios.  
   - **Líneas de demostración** — `.txt` pequeño con vectores de prueba públicos para verificar el análisis; sustituya en producción.  
   No existe aquí una «descarga pública de la Policía / Guardia Civil» como la del NSRL; las listas operativas las aporta la organización (véase [HASH_SETS.md](HASH_SETS.md)).

3. **Lista de alerta nivel evidencia**  
   - **Ninguno**, **Marcador vacío** o **Líneas de demostración** — misma idea que en sospechoso (`alert_evidence.txt`). Misma observación sobre fuentes de fuerzas de seguridad.

Si todos los niveles quedan vacíos en la práctica, el instalador escribe un `hashsets-config.json` **desactivado** (búsquedas desactivadas).

## Progreso, registro y finalización

- Durante la instalación, una **barra de progreso** refleja descarga, extracción, copia y pasos de bases de datos de hashes (hitos gruesos).  
- Las líneas de texto se añaden al **registro** en pantalla; los mismos mensajes se emiten con **`tracing`** (use `RUST_LOG`, p. ej. `RUST_LOG=info`).  
- Al terminar, un **diálogo nativo** informa de éxito o error (mensaje breve; el detalle permanece en el registro).

La interfaz está disponible en **inglés**, **español** y **francés** (botones de idioma en el aviso legal y la pantalla principal).

## Análisis frente a preparación

La **aplicación Foruster** no necesita Internet **durante el análisis**. Las búsquedas por hash usan solo archivos bajo su árbol portátil `data/hashsets/`. Las descargas grandes opcionales ocurren **aquí, en el instalador**, en el puesto de preparación, de forma coherente con [HASH_SETS.md](HASH_SETS.md).
