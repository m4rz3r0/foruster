# Bases de datos de hashes (NSRL y listas de alerta)

En el **triaje forense digital**, la **reducción de datos** —p. ej. filtrar software conocido frente a hallazgos prioritarios— es un criterio de eficiencia operativa. Foruster compara los resúmenes **MD5** y **SHA-1** de cada archivo escaneado con fuentes configurables:

- **Software conocido / comercial** — habitualmente la publicación **RDSv3** del [NSRL del NIST](https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds) en formato SQLite. Es el conjunto de datos público estándar para archivos de aplicaciones y SO «conocidos» que atenúa el ruido en escenarios de triaje.
- **Listas de alerta (dos niveles)** — **Sospechoso** y **Evidencia**, aportadas por su organización como listas de hashes en texto plano o bases SQLite con el mismo esquema estilo `FILE` que RDSv3. Foruster **no** incluye conjuntos de hashes de contenido ilícito de terceros; el manejo jurídico de ese material corresponde a quien despliega la herramienta.

**Idioma:** [English →](../en/HASH_SETS.md)

### ¿Por qué no hay una «descarga pública de la Policía / Guardia Civil»?

Los cuerpos nacionales de policía, la Guardia Civil y organismos similares **no** publican bases operativas de hashes para descarga pública sin restricciones del mismo modo que [el NIST publica el NSRL](https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds). Muchas categorías de alerta están **legalmente restringidas**, sujetas a **memorándums de entendimiento** o se distribuyen solo por **canales restringidos a fuerzas de seguridad** — no como enlaces HTTP anónimos dentro de un instalador genérico. Foruster mantiene por tanto esos niveles como **suministrados por la organización**: añada listas aprobadas bajo `data/hashsets/` según el procedimiento de su unidad, o comience con marcadores vacíos.

## Privacidad y seguridad

- Solo se comparan **hashes**; esta función no sube nada a la red.
- Las fuentes SQLite se abren **solo lectura**. Las listas planas se cargan en memoria; la SQLite RDSv3 se **consulta por archivo**, de modo que los conjuntos completos no tienen que residir en RAM.

## Examen sin conexión (uso forense habitual)

- **Durante el análisis**, Foruster **no** requiere Internet. Las búsquedas usan solo archivos bajo su árbol portátil `data/hashsets/` (p. ej. en USB), en coherencia con el requisito de **inferencia y cribado local** (*Edge AI* / reducción de datos sin filtrar evidencia hacia la nube).
- Las descargas grandes **opcionales** (p. ej. demo curada del NIST) ocurren **solo en el instalador**, en una **estación de preparación** con red. A continuación puede copiar el kit terminado a un entorno **aislado** (*air-gapped*) si su procedimiento pericial lo exige.

## Formatos admitidos

| Tipo | Extensiones | Comportamiento |
|------|---------------|----------------|
| NSRL RDSv3 | `.sqlite`, `.db` | Detecta una tabla con columnas `MD5` y `SHA-1` / `SHA1` (p. ej. `FILE`) y ejecuta `SELECT 1 … WHERE lower(hash)=?`. |
| Lista plana | `.txt`, otras | Un resumen **hexadecimal** por línea: **32** caracteres = MD5, **40** = SHA-1. Las líneas que empiezan por `#` son comentarios. |

## Almacenamiento portátil (forense / USB)

Todos los archivos de conjuntos de hashes y `hashsets-config.json` residen bajo **`data/hashsets/` junto al binario de Foruster** (el árbol de despliegue que copia al USB). **No** se escribe bajo el perfil de usuario del anfitrión (`~/.local/share`, `%APPDATA%`, etc.).

- Sobrescriba la raíz con la ruta absoluta `FORUSTER_PORTABLE_ROOT` si su diseño difiere (p. ej. `bin/` y `data/` no son hermanos).
- La disposición en disco queda definida por el esquema `hashsets-config.json` (configuración portátil de conjuntos de hashes).

### Instalador

El **foruster-installer** gráfico (ejecútelo solo en el puesto de preparación, no en la máquina examinada) configura **tres niveles** de forma independiente. Véase **[INSTALLER.md](INSTALLER.md)** para el comportamiento completo (modo sin conexión / en línea, progreso, registro).

**Conocidos buenos (estilo NSRL)**

| Opción | Comportamiento |
|--------|----------------|
| **Ninguno** | No hay archivo SQLite de conocidos buenos en la configuración (otros niveles pueden estar definidos). |
| **Muestra mínima** | Copia la SQLite RDSv3 incrustada (~8 KiB, `known_system.sqlite`) — funciona **sin red**. |
| **Demo curada NIST** | Descarga el zip público RDSv3 curado del NIST (~87 MiB). Requiere modo **en línea** en el instalador **solo en este PC de preparación**. |
| **Demo curada legada NIST** | Segundo zip curado desde una ruta antigua del NIST; requiere **en línea**. Si el NIST retira la URL, use otra opción o añada el archivo manualmente. |
| **NSRL completo (Android / Legacy / Modern minimal o Modern complete)** | Descarga un zip RDSv3 oficial multigigabyte del bucket S3 del NIST (versión fijada en el instalador, actualmente **2026.03.1**). Requiere **en línea**, diálogo de **confirmación** y comprobación de **espacio libre** (aprox. **2×** el tamaño publicado de la descarga). El NSRL **no** se genera localmente: solo lo **publica** el NIST. |

**Listas de alerta** de niveles sospechoso y evidencia

| Opción | Comportamiento |
|--------|----------------|
| **Ninguno** | Sin ruta para ese nivel. |
| **Marcador vacío (.txt)** | Crea `alert_suspicious.txt` o `alert_evidence.txt` solo con comentarios; añada después los hashes de la organización (mismo formato que en [Formatos admitidos](#formatos-admitidos)). |
| **Líneas de demostración** | `.txt` pequeño con vectores de prueba públicos para verificar el análisis; sustituya en producción. |

Si no se selecciona nada en ningún nivel, el instalador escribe `hashsets-config.json` con búsquedas **desactivadas**.

Tras la instalación, ejecute Foruster desde esa carpeta en el kit de examen; carga `data/hashsets/hashsets-config.json` **sin ninguna red**.

### Primer arranque sin instalador

Si aún no existe `hashsets-config.json`, Foruster puede sembrar una muestra **mínima** **solo** bajo `data/hashsets/` junto al ejecutable (misma regla portátil). Desactive con `FORUSTER_NO_DEFAULT_HASHSETS=1`.

### Verificación rápida

1. Confirme en **Ajustes** que las búsquedas por hash están activas y las rutas bajo su `data/hashsets/` portátil (USB).
2. Cree un archivo vacío en una carpeta escaneada: `touch …/nsrl-empty-test`
3. Ejecute el análisis; la cabecera de resultados **Hash DB** debe contar al menos un acierto **conocido bueno** para ese archivo.

### Descarga manual (alternativa al instalador)

```bash
chmod +x scripts/download-nsrl-curated-demo.sh
./scripts/download-nsrl-curated-demo.sh /ruta/al/USB/data/nsrl
```

Luego fije la ruta `.sqlite` en **Ajustes** y aplique. Fuente: [NIST — RDSv3 Demo Set](https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds).

## Configuración en tiempo de ejecución

La página **Ajustes** del escritorio permite:

- Activar o desactivar globalmente las búsquedas por hash.
- **Ocultar coincidencias conocidas buenas en los resultados de perfil** — si un archivo coincide con una fuente conocida buena (p. ej. NSRL), pueden suprimirse eventos de coincidencia de perfil para esa ruta y focalizar la lista en contenido relevante para el usuario.
- Rutas para fuentes conocidas buenas, lista de alerta sospechosa y lista de alerta de evidencia.
- **Aplicar** recarga todas las fuentes desde disco sin reiniciar la aplicación.

A nivel programático, el escritorio expone la configuración de conjuntos de hashes a través de la API de análisis pública (véase la documentación del SDK y del host en el árbol de desarrollo completo si dispone de acceso).

## Nota de arquitectura

Las búsquedas por hash se ejecutan en el **host** con acceso directo a archivos y SQLite opcional. **No** se implementan como plugins WASM: el modelo aislado de plugins no aplica a esta fase.

## Referencias

- NIST — *Current RDS Hash Sets* (RDSv3): https://www.nist.gov/itl/ssd/software-quality-group/national-software-reference-library-nsrl/nsrl-download/current-rds  
- Forensics Wiki — NSRL: https://forensics.wiki/national_software_reference_library/
