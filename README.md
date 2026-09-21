# Foruster

Adquisición forense de equipos en funcionamiento.

**[Ver la web](https://m4rz3r0.github.io/foruster/)** ·
[Documentación](https://m4rz3r0.github.io/foruster/documentacion/) ·
[Contacto](https://m4rz3r0.github.io/foruster/#contacto)

*Español · [English](README.en.md)*

---

Foruster copia la memoria y el almacenamiento de un equipo encendido sin
modificarlo, y deja un expediente que un tercero puede verificar por su cuenta.

Encontrar un archivo no basta. Hay que demostrar que es el mismo que estaba en el
dispositivo y que nadie lo tocó por el camino. De ahí sale la regla que ordena
todo lo demás: no se informa de nada que no se haya adquirido, sellado con su
hash y anotado antes en el expediente.

## Cómo trabaja

Los soportes se abren en solo lectura. Cada elemento se hashea con SHA-256
mientras se copia, no al terminar, y queda anotado en un manifiesto. El registro
de auditoría va encadenado por hash. Altera el expediente después de cerrarlo y
la cadena deja de cuadrar. Los traspasos de custodia se añaden a esa misma
cadena, y volver a comprobarla demuestra que sigue entera.

Sin número de expediente, organización, operador e identificador no arranca la
adquisición. Esos datos se escriben dentro del expediente, no se quedan en la
pantalla.

El orden lo impone la volatilidad, no la comodidad. Primero la memoria física del
equipo encendido, que desaparece al apagarlo; con la imagen viaja el contexto del
kernel sin el cual no se puede interpretar. Después, el almacenamiento que hayas
seleccionado.

No hace falta clonar un disco entero para trabajar. Foruster localiza artefactos
concretos por catálogo —cookies e historial de navegador, entre otros— con las
mismas garantías, y se lleva cada base de datos junto con sus ficheros
auxiliares, para que no acabes con una copia incompleta sin saberlo.

El manifiesto se firma con Ed25519, con la clave del laboratorio. Se comprueba
con `openssl`, sin ejecutar Foruster, así que la otra parte puede revisar el
trabajo sin dar por buena ninguna afirmación nuestra.

Del mismo expediente sale siempre el mismo informe, en texto y en PDF. Al
exportarlo no se recalcula nada, solo se lee lo que ya estaba guardado.

Y lo que salió mal consta. Un elemento que falla marca la sesión como parcial,
una capacidad que no está implementada lo dice en lugar de callarse, y las
limitaciones conocidas del análisis se declaran en el propio informe.

## Cómo se usa

Hay tres interfaces. La gráfica, para el trabajo de laboratorio y las actuaciones
en campo. La de terminal, para equipos sin entorno gráfico. Y la de línea de
comandos, que saca JSON y sirve para automatizar.

Se empieza de una de dos maneras. **Explorar** recorre los discos montados y
clasifica lo que hay sin copiar nada. **Adquirir** se lleva a un expediente lo
que hayas marcado. Lo normal es mirar antes de llevarse nada. Después vienen la
comprobación, la firma y el informe, en ese orden.

Funciona en Linux y en Windows. Las interfaces de terminal y de línea de comandos
son un único ejecutable estático, apto para un kit portátil.

## Qué no hace

- No está homologado, certificado ni validado por ningún organismo.
- La firma no es cualificada en el sentido de eIDAS, y no hay sellado de tiempo:
  la marca temporal no constituye fecha cierta.
- En Windows no adquiere memoria física.
- La imagen de memoria no es una instantánea de un instante único: el equipo
  sigue en marcha mientras se copia.
- No rompe el cifrado del sistema operativo.
- No detecta CSAM.
- No modifica el equipo examinado.

## Licencia

Software propietario. Todos los derechos reservados. Ver [`LICENSE`](LICENSE).

Escríbenos [desde la web](https://m4rz3r0.github.io/foruster/#contacto).
