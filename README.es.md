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

Toda la documentación publicada existe en **inglés** y **castellano**, con los **mismos nombres de archivo** en [`doc/en/`](doc/en/README.md) y [`doc/es/`](doc/es/README.md). Elija un idioma y use solo esa carpeta.

| Tema | English | Español |
|------|---------|---------|
| Punto de entrada | [doc/en/README.md](doc/en/README.md) | [doc/es/README.md](doc/es/README.md) |
| Índice | [doc/en/INDEX.md](doc/en/INDEX.md) | [doc/es/INDEX.md](doc/es/INDEX.md) |
| SDK WASM (API del host, ABI) | [doc/en/PLUGIN_SDK.md](doc/en/PLUGIN_SDK.md) | [doc/es/PLUGIN_SDK.md](doc/es/PLUGIN_SDK.md) |
| Guía de plugins | [doc/en/PLUGIN_DEVELOPMENT_GUIDE.md](doc/en/PLUGIN_DEVELOPMENT_GUIDE.md) | [doc/es/PLUGIN_DEVELOPMENT_GUIDE.md](doc/es/PLUGIN_DEVELOPMENT_GUIDE.md) |
| Modo forense / despliegue portátil | [doc/en/FORENSIC_POLICY.md](doc/en/FORENSIC_POLICY.md) | [doc/es/FORENSIC_POLICY.md](doc/es/FORENSIC_POLICY.md) |
| Compilación estática | [doc/en/STATIC_BUILDS.md](doc/en/STATIC_BUILDS.md) | [doc/es/STATIC_BUILDS.md](doc/es/STATIC_BUILDS.md) |
| Conjuntos de hashes | [doc/en/HASH_SETS.md](doc/en/HASH_SETS.md) | [doc/es/HASH_SETS.md](doc/es/HASH_SETS.md) |
| Instalador | [doc/en/INSTALLER.md](doc/en/INSTALLER.md) | [doc/es/INSTALLER.md](doc/es/INSTALLER.md) |
| Verificación de plugins WASM | [doc/en/PLUGIN_BUILD_VERIFY.md](doc/en/PLUGIN_BUILD_VERIFY.md) | [doc/es/PLUGIN_BUILD_VERIFY.md](doc/es/PLUGIN_BUILD_VERIFY.md) |

Los archivos que se publican en este repositorio público están **redactados**: no incluyen rutas internas del código ni detalles de infraestructura.

## Producto

Foruster es una aplicación de escritorio multiplataforma para **análisis forense en sistema en uso**: escaneo de almacenamiento activo, búsqueda por perfiles, funciones hash criptográficas, informes en PDF y un modelo de extensiones **WebAssembly** con aislamiento en el host.

La interfaz se construye con [Slint](https://slint.dev/) (véase la atribución arriba).

## Licencia

**Foruster** es **software propietario de código cerrado** en las entregas a licenciatarios o clientes. La documentación de este repositorio se ofrece como referencia; los componentes de terceros conservan sus licencias. No deduzca la licencia de un binario concreto solo a partir de este texto: consulte el aviso legal incluido con esa entrega.
