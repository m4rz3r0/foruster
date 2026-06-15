# Activos visuales del sitio público

Este directorio contiene los recursos estáticos de la página de producto (`docs/index.html`) y las guías generadas por MkDocs.

> **Nota importante:** los logos e iconos actuales (`foruster-wordmark.svg`, `foruster-compact.svg`, `foruster-icon.svg`, etc.) son **activos temporales** generados con asistencia de IA. Están sujetos a cambios futuros, incluido un posible rediseño de marca profesional. No deben considerarse definitivos ni reutilizarse como identidad corporativa estable fuera de este proyecto.

## Estructura

| Ruta | Uso |
|------|-----|
| `images/foruster-icon.svg` | Favicon y logo del tema MkDocs. |
| `images/foruster-wordmark.svg` | Marca completa en la portada (`index.html`). |
| `images/foruster-compact.svg` | Variante compacta de la marca. |
| `landing.css` | Estilos de la página de producto. |
| `landing.js` | Comportamiento de descargas e idioma de la portada. |

## Actualización

Si se actualizan los activos en `assets/brand/`, regenerar las copias aquí y volver a ejecutar `make public-docs-site` (o `scripts/sync-public-docs.sh`) para que el sitio publicado refleje los cambios.
