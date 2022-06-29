use crate::models::{casilla::Casilla, tablero::Dificultad};
use std::{convert::TryInto, io};
/*

Calcular numeros tiene que ser llamado cuando el usuraio introduzca
la posición de la bomba a desvelar. Si era una bomba, explota y pierde
si no, mira lo que hay a su inmediato alrededor, y si no hay ninguna bomba,
libera todo el tablero hasta que alguna casilla de las desbloqueadas tenga algo
adyacente, en cuyo caso lo revela y para

*/

pub fn numeros_contiguos(casillas: Vec<Casilla>) -> Vec<Casilla> {
    for (casilla, i) in casillas.iter().zip(0u64..) {
        Casilla::casillas_contiguas(i);
    }

    return casillas;
}


/*

    Esta función se encarga de interactuar con el usuario, saneando
    su input, para la elección de la dificultad de una nueva partida

*/

pub fn io_eleccion_dificultad() -> Dificultad {

    loop {
        println!(
            "Dificultad\n\
    \n\
    1. Fácil\n\
    2. Normal\n\
    3. Difícil \n\
    \n"
        );

        let mut dificultad_str = String::new();

        io::stdin()
            .read_line(&mut dificultad_str)
            .expect("Ups! No te entiendo");

        let dificultad = Dificultad::from_str(dificultad_str);

        match dificultad {
            Ok(x) => return x,
            Err(_) => println!("Eso no es una dificultad. Escoge una dificultad de la lista"),
        }
    }
}


/*

    Esta función se encarga de interactuar con el usuario, saneando
    su input, para la elección de la casilla a destapar en el turno del jugador

*/

pub fn io_eleccion_casilla((ancho_tablero, largo_tablero): (u16, u16)) -> (u64, u64) {
    let (mut fila_input, mut columna_input): (String, String) = ("0".to_string(), "0".to_string());

    println!("¿Qué casilla quieres destapar?");

    // Comprobamos caracter válido + lógica de saneo de input arbitraria
    // En este caso, comprobamos que la fila exista en el tablero
    loop {
        println!("Fila: ");

        match io::stdin().read_line(&mut fila_input) {
            Ok(x) => {
                if x < largo_tablero.try_into().unwrap() {
                    break;
                } else {
                    println!(
                        "Por favor, escoge una fila entre la 1 y la {}",
                        largo_tablero
                    )
                };
            }
            Err(_) => println!("Ups! No te entiendo :("),
        }
    }

    loop {
        println!("Columna: ");

        match io::stdin().read_line(&mut columna_input) {
            Ok(x) => {
                if x < ancho_tablero.try_into().unwrap() {
                    break;
                } else {
                    println!(
                        "Por favor, escoge una columna entre la 1 y la {}",
                        ancho_tablero
                    );
                };
            }
            Err(_) => println!("Ups! No te entiendo :("),
        }
    }

    // Convertimos de String a u64
    return (
        str::parse::<u64>(fila_input.as_str()).unwrap(),
        str::parse::<u64>(columna_input.as_str()).unwrap(),
    );
}

pub fn fila_col_a_indice((fila_casilla, columna_casilla) : (u16, u16), (ancho_tablero, largo_tablero) : (u16, u16)) -> u64{
    return ((fila_casilla - 1) * ancho_tablero - columna_casilla + 1).into();
}
