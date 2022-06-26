use crate::models::casilla::Casilla;


// Calcular numeros tiene que ser llamado cuando el usuraio introduzca
// la posición de la bomba a desvelar. Si era una bomba, explota y pierde
// si no, mira lo que hay a su inmediato alrededor, y si no hay ninguna bomba,
// libera todo el tablero hasta que alguna casilla de las desbloqueadas tenga algo
// adyacente, en cuyo caso lo revela y para-

pub fn numeros_contiguos(casillas: Vec<Casilla>) -> Vec<Casilla> {
    for (casilla, i) in casillas.iter().zip(0u64..) {
        casillas_contiguas(i);
    }

    return casillas;
}

pub fn casillas_contiguas(indice: u64) -> Vec<u64> {
   
    let vec = vec![1u64, 22u64];
    vec

    // Los numeros en las casillas no se calculan a priori, sino que se generaran
    // en tiempo de ejecución, en el momento que se selecciona una casilla se comprueban
    // las adyacentes y, no antes.
    // 1   2   3   --> Orden en el que se almacenan las casillas
    // 4   X   5   --> adyacentes a la casilla clicada "X" en el
    // 6   7   8   --> vector de casillas contiguas

    // Indices para cada una de las posiciones:

    // 1. X - 7 - 2
    // 2. X - 7 - 1
    // 3. X - 7
    // 4. X - 1
    // 5. X + 1
    // 6. X + 7
    // 7. X + 7 + 1
    // 8. X + 7 + 2

    // Quitarle 7 es subir una fila
    // Sumarle 7 es bajar una fila
}

pub fn es_borde(indice: u32, cantidad_bombas: u32) -> bool {
    return true; //let ancho = //sqrt(cantidad_bombas)
                 // En un tablero de tamaño arbitrario, los bordes y esquina son:
                 // 0, x + (cantidad de bombas de ancho) * i, i entre 1
                 //cantidad de bombas de ancho se averigua haciendo la sqrt a la length del array
                 // que contiene las
}
