# Buscaminas in CLI


Los imports en Rust funcionan de la siguiente manera:

La existencia de los módulos no viene dada implícitamente como en otros lenguajes, sino que tiene que ser declarada.
Independientemente de la estructura de ficheros de nuestro proyecto, el único crate que Rust ve a proiri es main.
Desde main se podrán conocer el resto de módulos, actuando este como punto de entrada.

Existen dos formas idiomáticas de declarar módulos en Rust, prefiriéndose la segunda:

1. Se crea un fichero mod.rs en el mismo directorio que el módulo a incluir, cuyo contenido referencia al fichero:
   "pub mod fichero_modulo;"

2. Se crea un fichero, en la raíz, que se llame igual que el directorio que contiene los módulos (como un intermediaro entre los módulos en un directorio, y main) y dentro se importa cada módulo. Así es tal y como está hecho en el proyecto
