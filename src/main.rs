// Se importa el modulo
mod tablero;

fn main() {

    println!
    (">>>>>>>>> BUSCAMINAS <<<<<<<<<\n\
    \n");

    let mut tablero = tablero::Tablero::construir_tablero(tablero::Dificultad::Facil);
    
    // EL BUSCAMINAS
    //1. Dar la bienvenida
    //2. Preguntar el nivel de dificultad y tomar el input de usuario
    //3. Imprimir el tablero
    //4. 

    // Niveles de dificultad:
    // Principiante: 8 x 8 -> 10 minas
    // Intermedio: 16 x 16 -> 40 minas
    // Experto: 16 x 30 -> 99 minas
    


    // Las casillas tienen asociadas o no una bomba.
    // Si no tienen una bomba, tienen un número que indica las bombas colindantes que hay.
    // Puede ser 0, así que se desbloquearía automáticamente

    
}
