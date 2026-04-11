# Foruster — documentación (español)

**Idioma:** [English →](../en/README.md)

Foruster es un **sistema forense inteligente** para el **triaje forense digital** en **sistemas en uso**: orienta la actuación hacia la evaluación preliminar *in situ* frente al análisis exclusivamente *post-mortem* en laboratorio, e integra **visión artificial** y **aprendizaje profundo** mediante **inferencia en el propio equipo**, sin enviar **evidencia digital** a servicios remotos.

La documentación se publica en **inglés y español en paralelo** (mismos nombres de archivo en [`doc/en/`](../en/) y [`doc/es/`](.)). Conviene no mezclar idiomas entre carpetas.

| Documento | Descripción |
|-----------|-------------|
| [INDEX.md](INDEX.md) | Índice temático (plugins, SDK, política forense, instalador). |
| [PLUGIN_SDK.md](PLUGIN_SDK.md) | Contrato del SDK, ABI y aplicación **anfitriona**. |
| [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md) | Guía para **plugins WASM** (módulos de análisis). |
| [FORENSIC_POLICY.md](FORENSIC_POLICY.md) | Política de operación, **cadena de custodia** y kit portátil. |
| [STATIC_BUILDS.md](STATIC_BUILDS.md) | Binarios nativos, enlazado y **motor de inferencia** (ONNX Runtime). |
| [HASH_SETS.md](HASH_SETS.md) | Bases de datos de hashes (NSRL, listas de alerta). |
| [INSTALLER.md](INSTALLER.md) | Instalador (preparación del kit en laboratorio). |
| [PLUGIN_BUILD_VERIFY.md](PLUGIN_BUILD_VERIFY.md) | Compilar y verificar plugins WASM. |

El [README](../../README.md) en la raíz describe licencia y publicaciones. La terminología forense sigue el *Trabajo Fin de Máster* (Universidad de Extremadura, 2026).
