# Guía de instalación y preparación del kit

La utilidad **`foruster-installer`** es una ventana gráfica para **preparar el kit de despliegue**: copia Foruster y los recursos opcionales (conjuntos de hashes, modelos, etc.) a un **medio de destino** (por ejemplo un USB preparado en laboratorio o en una estación de trabajo).

!!! danger "No ejecute el instalador en el equipo bajo examen"
    Úselo **solo en el equipo de preparación**. En la máquina analizada pueden producirse descargas, escrituras o cambios de configuración que **afectan a la cadena de custodia**.

!!! tip "Para quién es esta guía"
    Si solo va a **abrir un paquete ya descargado** (carpeta con el programa dentro), puede bastar con descomprimirlo. Esta página detalla el **instalador gráfico** cuando quiere **elegir versiones**, **bases de hashes** y **destino** de forma guiada.

---

## Resumen: qué va a hacer

| Fase | Qué ocurre |
|------|------------|
| 1. Origen | Indica si los archivos vienen **de esta carpeta** (sin red) o si el programa debe **descargarlos** (con red, solo aquí). |
| 2. Versión y extensiones | En modo con red, puede fijar **qué publicación** de la aplicación y de extensiones instalar. |
| 3. Destino | Elige la **carpeta o disco** donde quedará el kit (por ejemplo la unidad USB). |
| 4. Hashes opcionales | Decide **NSRL** y **listas de alerta** que se copian o descargan al árbol `data/hashsets/`. |

Al terminar, tendrá una carpeta **portátil**: puede ejecutar Foruster desde ahí en el equipo previsto, según su protocolo.

---

## Antes de empezar (lista corta)

- **Espacio en disco**: si va a descargar bases grandes (NSRL completas), reserve **varios gigabytes** libres en el destino; el instalador comprueba espacio aproximado.
- **Red**: el modo **en línea** solo debe usarse en el **puesto de preparación** con red permitida por su organización.
- **Paquete local**: en modo **sin conexión**, coloque junto al instalador el **paquete** con `foruster` / `foruster.exe` y las carpetas de soporte, o defina la variable `FORUSTER_BUNDLE_ROOT` si el paquete está en otra ruta.

---

## Modos de origen

| Modo | Uso |
|------|-----|
| **Sin conexión** | El instalador está junto a un **paquete** local (`foruster` / `foruster.exe` y carpetas de soporte). No se descarga nada. |
| **En línea** | Obtiene paquetes de publicación y manifiestos de extensiones desde GitHub (TLS). Requiere red **solo en este equipo de preparación**. |

Defina `FORUSTER_BUNDLE_ROOT` si el paquete no está junto al ejecutable del instalador.

---

## Versiones (modo en línea)

Este bloque es **siempre visible**. En modo **Sin conexión** los controles están **deshabilitados** (y la tarjeta se atenúa) para que cambiar el ORIGEN **no** desplace el resto del formulario — mejor accesibilidad y disposición predecible. Cuando el ORIGEN es **En línea**, active **Mostrar selección de versiones** para elegir:

- **Publicación de la aplicación** — Etiquetas que publican un paquete por plataforma para el SO elegido (o **Última** para la publicación predeterminada).
- **Publicación de extensiones** — Una etiqueta `plugins/…` para `plugins-manifest.json` (o **Últimos plugins**).

Use **Aplicar** junto a la lista de extensiones para volver a cargar el manifiesto tras cambiar la etiqueta de extensiones. **Actualizar listas de versiones** vuelve a consultar las etiquetas (p. ej. tras una nueva publicación).

---

## Plataforma de destino

Elija **Linux** o **Windows** para que el nombre del binario instalado sea coherente con la **instalación de destino** (convenciones de cada plataforma). Cambiar el objetivo actualiza la lista de etiquetas de la **aplicación** cuando hay conexión.

---

## Bases de datos de hashes (`data/hashsets/`)

El instalador escribe todo bajo la **carpeta de destino** elegida — diseño portátil: `data/hashsets/` junto a la aplicación. No se guarda nada bajo el perfil de usuario del sistema donde se instala.

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
   No existe aquí una «descarga pública de la Policía / Guardia Civil» como la del NSRL; las listas operativas las aporta la organización (véase [Bases de datos de huellas (NSRL y listas)](HASH_SETS.md)).

3. **Lista de alerta nivel evidencia**  
   - **Ninguno**, **Marcador vacío** o **Líneas de demostración** — misma idea que en sospechoso (`alert_evidence.txt`). Misma observación sobre fuentes de fuerzas de seguridad.

Si todos los niveles quedan vacíos en la práctica, el instalador escribe un `hashsets-config.json` **desactivado** (búsquedas desactivadas).

---

## Progreso, registro y finalización

- Durante la instalación, una **barra de progreso** refleja descarga, extracción, copia y pasos de bases de datos de hashes (hitos gruesos).  
- Las líneas de texto se añaden al **registro** en pantalla; los mismos mensajes se emiten con **`tracing`** (use `RUST_LOG`, p. ej. `RUST_LOG=info`).  
- Al terminar, un **diálogo nativo** informa de éxito o error (mensaje breve; el detalle permanece en el registro).

La interfaz del instalador está disponible en **inglés**, **español** y **francés** (botones de idioma en el aviso legal y la pantalla principal).

---

## Análisis frente a preparación

La **aplicación Foruster** no necesita Internet **durante el análisis**: las búsquedas por hash usan solo los archivos de `data/hashsets/` **en el kit portátil**. Las descargas grandes opcionales se hacen **desde el instalador**, en el equipo de preparación; los detalles conceptuales están en [Bases de datos de huellas (NSRL y listas)](HASH_SETS.md).

---

## Más ayuda

- [Preguntas frecuentes](FAQ.md) — qué archivo descargar y dudas habituales.  
- [Política de uso forense y modo portátil](FORENSIC_POLICY.md) — dónde escribe el programa en modo forense.

---

[Descargas e inicio de la aplicación](https://m4rz3r0.github.io/foruster/)
