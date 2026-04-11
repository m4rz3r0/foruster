<div align="center">
  <h1>Foruster</h1>
  <p align="center"><strong>Triaje forense en vivo y detección de anomalías</strong></p>
  <p align="center">
    <a href="https://slint.dev">
      <img alt="#MadeWithSlint" src="https://raw.githubusercontent.com/slint-ui/slint/master/logo/MadeWithSlint-logo-light.svg" height="48">
    </a>
  </p>
</div>

---

**Idiomas:** [English](README.md)

Este repositorio es el **canal público de documentación y de publicación** de **Foruster**. **El código fuente de la aplicación no se publica aquí.** El desarrollo canónico reside en una forja privada; este espacio se limita a documentación apta para distribución y a binarios asociados a [Releases](https://github.com/m4rz3r0/foruster/releases).

## Documentación

| Tema | Documento |
|------|-----------|
| Punto de entrada (inglés / español) | [doc/README.md](doc/README.md) |
| Índice en castellano | [doc/es/INDICE.md](doc/es/INDICE.md) |
| Índice técnico (inglés) | [doc/INDEX.md](doc/INDEX.md) |
| SDK WASM (API del host, ABI) | [doc/PLUGIN_SDK.md](doc/PLUGIN_SDK.md) |
| Guía de plugins | [doc/PLUGIN_DEVELOPMENT_GUIDE.md](doc/PLUGIN_DEVELOPMENT_GUIDE.md) |
| Modo forense / despliegue portátil | [doc/FORENSIC_POLICY.md](doc/FORENSIC_POLICY.md) |
| Compilación estática | [doc/STATIC_BUILDS.md](doc/STATIC_BUILDS.md) |
| Conjuntos de hashes | [doc/HASH_SETS.md](doc/HASH_SETS.md) |
| Instalador | [doc/INSTALLER.md](doc/INSTALLER.md) |
| Verificación de plugins WASM | [doc/PLUGIN_BUILD_VERIFY.md](doc/PLUGIN_BUILD_VERIFY.md) |

Los archivos copiados al espejo público están **redactados**: no incluyen rutas internas del repositorio ni detalles de infraestructura. Parte de la documentación de referencia está en **inglés**; el índice en español indica el idioma de cada guía.

## Producto

Foruster es una aplicación de escritorio multiplataforma para **análisis forense en sistema en uso**: escaneo de almacenamiento activo, búsqueda por perfiles, funciones hash criptográficas, informes en PDF y un modelo de extensiones **WebAssembly** con aislamiento en el host.

La interfaz se construye con [Slint](https://slint.dev/) (véase la atribución arriba).

## Licencia

**Foruster** es **software propietario de código cerrado** en las entregas a licenciatarios o clientes. La documentación de este repositorio se ofrece como referencia; los componentes de terceros conservan sus licencias. No deduzca la licencia de un binario concreto solo a partir de este texto: consulte el aviso legal incluido con esa entrega.
