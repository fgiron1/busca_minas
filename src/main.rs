use std::io;

pub mod models;
pub mod utils;

use models::tablero::Dificultad;
use models::tablero::Tablero;

use utils::io_eleccion_dificultad;
use utils::io_eleccion_casilla;


// Se importa el modulo


fn main() {

    // Niveles de dificultad:
    // Principiante: 8 x 8 -> 10 minas
    // Intermedio: 16 x 16 -> 40 minas
    // Experto: 16 x 30 -> 99 minas
    
    println!
    (">>>>>>>>> BUSCAMINAS <<<<<<<<<\n\n");

    // Se le pide al usuario que escoja una dificultad entre las 3 anteriores.
    // Se recoge y sanea su input de usuario

    let dificultad = io_eleccion_dificultad();
    let mut tablero = Tablero::construir_tablero(dificultad);
    
    // Bucle del juego

    loop {

        // Se le muestra el tablero al usuario y se le pide que escoja una casilla
        // a destapar. Se recoge y sanea el input de usuario.

        tablero.mostrar();
        let (fila_eleccion, columna_eleccion) = io_eleccion_casilla((tablero.ancho, tablero.largo));

        

    }
    

    
    

    // EL BUSCAMINAS
    //1. Dar la bienvenida
    //2. Preguntar el nivel de dificultad y tomar el input de usuario
    //3. Imprimir el tablero
    //4. Comenzar el bucle del juego
    //5. 

    // BUCLE DEL JUEGO
    // 1. Se le pide el input de usuario al jugador
    // 2. Usuario dice: Fila 3, columna 4 (indice 27):
    // 3. Calculamos el indice: (fila * 8) + (col - 1)
    // 4. Llamamos a numeros_contiguos(27) que devuelve un &Vec<Casilla> (un slice)
    // 5. Se llama a actualizar_tablero(), encargado de desvelar las casillas que no tengan nada,
    //    sucesivamente
    // 5. Se actualiza el tablero con esa nueva información de las casillas
    // 6. Se muestra el nuevo tablero
    // 7. Si ha perdido, se le dice (habia bomba). Si no hay bomba, se comprueba que
    // esten todas las casillas revealed. Esta comprobación se hace después de todo movimiento
    
    // 8)


    // Las casillas tienen asociadas o no una bomba.
    // Si no tienen una bomba, tienen un número que indica las bombas colindantes que hay.
    // Puede ser 0, así que se desbloquearía automáticamente

    
}
