# Foruster — sitio público (GitHub Pages)

Este directorio es lo que debe publicarse en **https://m4rz3r0.github.io/foruster/** cuando **GitHub Pages** toma la rama **`main`** y la carpeta **`/docs`** del repositorio público **github.com/m4rz3r0/foruster**.

## Contenido

| Ruta | Función |
|------|---------|
| **`index.html`** | Portada: presentación, enlace a la última versión (interfaz de la API de GitHub) y acceso a las guías en `guide/`. |
| **`guide/`** | Sitio **estático** generado con **MkDocs Material**. Las entradas por idioma están en `guide/es/` y `guide/en/`. Debe **versionarse** aquí para que las páginas funcionen sin ejecutar MkDocs en el servidor. |
| **`documentation/en/`**, **`documentation/es/`** | Copia de las guías incluidas en la lista permitida (`doc/public-allowlist.txt`), fuente de MkDocs; también versionada para builds reproducibles. |
| **`mkdocs.yml`**, **`requirements-docs.txt`**, **`documentation/index.md`** | Configuración y página intermedia de MkDocs. |
| **`assets/landing.css`**, **`assets/landing.js`** | Estilos y comportamiento solo de la portada. |
| **`.nojekyll`** | Impide que Jekyll procese el sitio en GitHub Pages. |

No hay carpetas `en/` ni `es/` sueltas en la raíz de `docs/`; lo publicado queda bajo **`documentation/`** y **`guide/`**.

## Cambios en las guías (repositorio de desarrollo)

1. Editar el Markdown canónico en **`doc/en/`** y **`doc/es/`** (no limitarse a la copia bajo `documentation/`).
2. Regenerar y confirmar el resultado en `docs/`:

```bash
make public-docs-site
git add doc/ docs/documentation/ docs/guide/
git commit -m "docs: …"
```

3. Enviar la rama que alimenta GitHub (`main` u otra acordada) al remoto **github.com/m4rz3r0/foruster**.

La integración continua en Forgejo puede ejecutar `scripts/sync-public-docs.sh --push-github` para volcar el mismo árbol al repositorio público; el contenido de `docs/` en el repositorio de desarrollo debe coincidir con lo generado para no desalinear fuentes y HTML.

## Vista previa local

```bash
make public-docs-site
```

Abra `docs/guide/index.html` o sirva `docs/` con un servidor HTTP estático. Requiere MkDocs Material (`pip install -r docs/requirements-docs.txt` o el entorno equivalente en su sistema).
