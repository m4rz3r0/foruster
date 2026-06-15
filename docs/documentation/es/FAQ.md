# Preguntas frecuentes

## ¿Qué es Foruster?

Foruster es una aplicación de escritorio multiplataforma para **triaje forense en sistemas en uso**: recorre el almacenamiento, permite filtrar por perfiles de fichero, calcula huellas criptográficas, genera informes en PDF y puede cargar **extensiones** aisladas en el equipo mediante WebAssembly.

---

## ¿Necesito una cuenta en GitHub para descargar?

**No.** En la sección de **descargas** del **[sitio web del proyecto](https://m4rz3r0.github.io/foruster/)** figuran los archivos de la última publicación; basta con un navegador.

---

## ¿Debo usar la línea de órdenes?

**No para el uso básico.** Puede trabajar con el **gestor de archivos** y la **ventana principal** del programa. La línea de órdenes solo resulta necesaria si su organización exige opciones avanzadas (por ejemplo variables de entorno descritas en la [política de uso forense](FORENSIC_POLICY.md)).

---

## ¿Qué archivo debo descargar?

- **Windows** — El **paquete autónomo** (archivo `.zip` que reúne aplicación, modelos y extensiones de ejemplo) suele ser lo más cómodo. El **programa instalador** (ejecutable `.exe` cuyo nombre indica instalador) sirve para preparar un árbol portátil en un soporte extraíble. El ejecutable **`foruster-…-windows-x64.exe`** sin la mención a instalador es el binario de la aplicación para el uso habitual.
- **GNU/Linux** — Misma lógica: **paquete autónomo** (`.tar.gz` análogo), **instalador** o binario **`foruster-…-linux-x64`**.

Para comprobar la integridad tras la descarga, use el fichero **`SHA256SUMS`** de la misma publicación.

---

## ¿Es software libre?

**No.** Foruster es **software propietario**. Las condiciones exactas constan en el material que descarga y en los enlaces de licencia del **[sitio web del proyecto](https://m4rz3r0.github.io/foruster/)**.

---

## ¿Dónde está la guía del instalador?

La descripción paso a paso del instalador gráfico está en [Instalación y preparación del kit](INSTALLER.md). El [centro de documentación](README.md) enlaza el resto de materias (política forense, bases de huellas, índice).

---

## ¿Puedo usar un soporte preparado en otro ordenador?

Sí: el diseño es **portátil**. El kit debe incluir la aplicación y la carpeta `data/` tal como los preparó con el [instalador](INSTALLER.md). Consulte siempre la [política de uso forense](FORENSIC_POLICY.md) de su procedimiento para saber **dónde** puede ejecutarse el programa.

---

## ¿El programa envía mis archivos a Internet?

**No** durante el análisis habitual: las comparaciones por huellas y el trabajo principal son **locales**. Las **descargas voluminosas** (por ejemplo bases NSRL) se realizan **solo desde el instalador** en el puesto de preparación, no en el equipo bajo examen. Más detalle en [Bases de datos de huellas (NSRL y listas)](HASH_SETS.md).

---

[Descargas e inicio](https://m4rz3r0.github.io/foruster/)
