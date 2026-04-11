# Foruster — documentación (español)

**Idioma:** [English →](../en/README.md)

Foruster es un **sistema forense inteligente** para el **triaje forense digital** en **sistemas en uso**: frente a un análisis exclusivamente *post-mortem* en laboratorio, orienta la actuación hacia la evaluación preliminar *in situ* y permite integrar **visión artificial** y **aprendizaje profundo** mediante **inferencia local** —**inteligencia artificial en el borde** (*Edge AI*)— sin que la **evidencia digital** abandone el equipo hacia servicios en la nube.

La documentación publicada se organiza en **dos árboles paralelos** (los mismos nombres de archivo en **[`doc/en/`](../en/)** y **[`doc/es/`](.)**). Conviene elegir un idioma y no mezclar ambos.

| Documento | Descripción |
|-----------|-------------|
| [INDEX.md](INDEX.md) | Índice temático (plugins, SDK, política de operación forense, instalador). |
| [PLUGIN_SDK.md](PLUGIN_SDK.md) | Contrato del SDK, ABI y capa **host** (anfitrión). |
| [PLUGIN_DEVELOPMENT_GUIDE.md](PLUGIN_DEVELOPMENT_GUIDE.md) | Guía para **plugins WASM** (módulos de análisis). |
| [FORENSIC_POLICY.md](FORENSIC_POLICY.md) | Política de operación, **cadena de custodia** y despliegue portátil. |
| [STATIC_BUILDS.md](STATIC_BUILDS.md) | Binarios nativos, enlazado y **motor de inferencia** (ONNX Runtime). |
| [HASH_SETS.md](HASH_SETS.md) | Bases de datos de hashes (NSRL, listas de alerta). |
| [INSTALLER.md](INSTALLER.md) | Instalador gráfico (estación de preparación del kit). |
| [PLUGIN_BUILD_VERIFY.md](PLUGIN_BUILD_VERIFY.md) | Compilar y verificar extensiones WASM. |

El [README](../../README.md) en la raíz describe licencia y publicaciones. La terminología forense de estas páginas sigue la del *Trabajo Fin de Máster* (Universidad de Extremadura, 2026).
