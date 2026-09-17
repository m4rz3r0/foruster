# Foruster

**Adquisición y análisis forense de almacenamiento.**

Herramienta de escritorio para unidades de Policía Judicial y peritos
informáticos. Adquiere evidencia digital de un sistema en funcionamiento sin
modificarlo, y deja constancia verificable de qué se recogió, cuándo, por quién
y con qué integridad.

*Español · [English](README.en.md)*

---

## Propósito

En un procedimiento judicial no basta con encontrar un archivo: hay que poder
demostrar que es el mismo que estaba en el dispositivo y que nadie lo alteró por
el camino. Foruster se diseñó alrededor de esa exigencia.

El principio que lo ordena todo: **no se informa de nada que no se haya
adquirido, hasheado y registrado antes en un contenedor auditable.**

## Qué hace

**Adquisición con cadena de custodia.** Cada elemento se lee en solo lectura, se
hashea con SHA-256 mientras se copia y se anota en un manifiesto. El registro de
auditoría va encadenado por hash, de modo que cualquier alteración posterior
resulta detectable. Los traspasos de custodia se añaden a esa misma cadena y la
reverificación demuestra que no se rompió.

**Identidad del caso obligatoria.** Sin número de expediente, organización,
operador e identificador, la adquisición no arranca. Esos datos viven dentro del
contenedor, no en la interfaz.

**Memoria volátil.** Adquiere la memoria física de un equipo encendido sin
instalar nada en él, en formato estándar legible por herramientas de análisis de
terceros. Con la imagen viaja el contexto del kernel sin el cual no se puede
interpretar, y que desaparece al apagar el equipo.

**Triaje dirigido.** En lugar de clonar un disco completo, localiza y recoge
artefactos concretos por catálogo —cookies e historial de navegador, entre
otros— aplicando las mismas garantías. Las bases de datos viajan siempre con sus
ficheros auxiliares, para que no se adquiera una copia incompleta sin saberlo.

**Firma del manifiesto.** Firma Ed25519 con la clave del laboratorio.
Verificable por un tercero con herramientas estándar, sin necesidad de ejecutar
Foruster: un perito de la parte contraria puede comprobar el trabajo por su
cuenta.

**Informe determinista.** El mismo contenedor produce siempre el mismo informe,
en texto y en PDF. Nunca se recalcula nada al exportar.

**Honestidad sobre lo incompleto.** Un elemento que falla marca la sesión como
parcial; una capacidad no implementada lo dice; y las limitaciones conocidas del
análisis se declaran en el propio informe en lugar de omitirse.

## Cómo se usa

Tres interfaces sobre el mismo núcleo forense:

| | Para qué |
|---|---|
| **Gráfica** | Trabajo habitual de laboratorio y actuaciones en campo. |
| **Terminal** | Equipos sin entorno gráfico. Binario único, sin instalación. |
| **Línea de comandos** | Automatización e integración con los sistemas del laboratorio. Salida en JSON. |

Flujo típico: seleccionar objetivos → adquirir con los datos del caso →
verificar → firmar → exportar el informe pericial.

Funciona en Linux y Windows. Las interfaces de terminal y línea de comandos se
distribuyen como binario estático, apto para un kit portátil.

## Qué no hace

Se indica de forma explícita porque importa antes de citar la herramienta en un
dictamen:

- **No está homologada, certificada ni validada oficialmente** por ningún
  organismo.
- **No emite firma cualificada** en el sentido de eIDAS, ni sellado de tiempo:
  su marca temporal no constituye fecha cierta.
- **No adquiere memoria en Windows**, ni rompe el cifrado del sistema operativo.
- **La imagen de memoria no es una instantánea de un instante único**: el equipo
  sigue en marcha mientras se copia.
- **No detecta CSAM.**
- **No modifica** el sistema examinado.

## Disponibilidad

Producto en desarrollo. Para demostraciones, evaluación o condiciones de
licencia, escriba a la dirección de contacto del perfil.

## Licencia

Software propietario. Todos los derechos reservados. Consulte [`LICENSE`](LICENSE).

El código fuente no se publica. Este repositorio contiene únicamente
información de producto.
