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

Sin número de expediente, organización, operador, identificador y habilitación
no arranca la adquisición. Esos datos se escriben dentro del expediente, no se
quedan en la pantalla. El alcance autorizado es una restricción técnica: lo que
no esté autorizado se rechaza antes de crear el expediente.

El orden lo impone la volatilidad, no la comodidad. Primero la memoria del
equipo encendido, que desaparece al apagarlo: la física en Linux, con el
contexto del kernel sin el cual la imagen no se puede interpretar, y la de los
procesos en Windows. Después, el almacenamiento que hayas seleccionado.

Cuando hace falta el soporte entero, Foruster toma la imagen de un volumen, de
un disco o de una instantánea de Windows que ya exista, en raw o en E01. La
imagen de un soporte en uso no es atómica, y la sesión nunca figura como
completa. En Windows recoge también las colmenas del registro, los registros
de eventos, `$MFT` y otros artefactos leyendo el NTFS en crudo, incluidos los
ficheros que el sistema mantiene bloqueados. Los recoge; no los interpreta.

No siempre hace falta clonar un disco entero. Foruster localiza artefactos
concretos por catálogo —cookies e historial de navegador, entre otros— con las
mismas garantías, y se lleva cada base de datos junto con sus ficheros
auxiliares, para que no acabes con una copia incompleta sin saberlo.

El manifiesto se firma con Ed25519, con la clave del laboratorio, y admite un
sello de tiempo RFC 3161. La firma se comprueba con `openssl` y un E01 con
`ewfverify`, sin ejecutar Foruster, así que la otra parte puede revisar el
trabajo sin dar por buena ninguna afirmación nuestra.

Del mismo expediente sale siempre el mismo informe, en texto y en PDF, y el acta
de volcado. Al exportarlos no se recalcula nada, solo se lee lo que ya estaba
guardado.

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
son un único ejecutable estático, apto para un kit portátil. La licencia se
comprueba sin conexión, y solo la adquisición la exige. El destino de cada
expediente va siempre en otro dispositivo, nunca en el equipo intervenido.

## Qué no hace

- No está homologado, certificado ni validado por ningún organismo, y no
  garantiza la validez de la prueba.
- La firma no es cualificada en el sentido de eIDAS. El sello de tiempo solo es
  cualificado si la autoridad de sellado es un prestador cualificado.
- No comprueba la firma de la autoridad de sellado; eso se hace con
  `openssl ts -verify`. Solo pide el sello directamente a una autoridad
  `http://`; con una `https://`, la respuesta se obtiene aparte y se carga.
- No comprueba las firmas externas que se adjuntan al expediente: las registra.
- No extrae material de clave de BitLocker ni de LUKS.
- En Windows no adquiere memoria física, sino la de los procesos.
- Una imagen de memoria nunca está completa: el equipo sigue en marcha mientras
  se copia.
- No interpreta los artefactos de Windows que recoge.
- No rompe el cifrado del sistema operativo.
- No detecta CSAM.
- No modifica el equipo intervenido.

## Licencia

Software propietario. Todos los derechos reservados. Ver [`LICENSE`](LICENSE).

Escríbenos [desde la web](https://m4rz3r0.github.io/foruster/#contacto). Para
avisar de una vulnerabilidad, consulta la
[página de seguridad](https://m4rz3r0.github.io/foruster/documentacion/seguridad/).
