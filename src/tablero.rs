use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Dificultad {
    Facil,
    Normal,
    Dificil
}

#[derive(Debug)]
pub struct InputError;

impl Dificultad {
    pub fn from_str(s: String) -> Result<Dificultad, InputError> {
        return match s.as_str() {
            "fácil" => Ok(Dificultad::Facil),
            "normal" => Ok(Dificultad::Normal),
            "dificil" => Ok(Dificultad::Dificil),
            _ => Err(InputError)
            
        };
    }
}

#[derive(PartialEq)]
pub enum Contenido {
    Number(u8),
    Bomba,
    None,
}

pub struct Casilla {
    contenido: Contenido,
    revelada: bool,
}

impl Casilla {
    fn nueva(bomba: bool) -> Casilla {

        let casilla : Casilla = Casilla {
            contenido: if bomba {Contenido::Bomba} else {Contenido::None},
            revelada: false,
        };
        
        return casilla;
    }
}

impl fmt::Display for Casilla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        
        let imprimir;

        if (self.revelada){
            imprimir = match self.contenido {
                Contenido::None => " ",
                Contenido::Bomba => "X",
                Contenido::Number(x) => &x.to_string()
            };
        } else {
            //Casilla oculta
            imprimir = "@"
        }

        write!(f, "{}", imprimir)
    }
}

pub struct Tablero {
    ancho: u16,
    largo: u16,
    casillas: Vec<Casilla>,
    es_ganador: bool,
}

impl Tablero {

    pub fn construir_tablero(dificultad : Dificultad) -> Tablero {
        
        let (ancho, largo, numero_bombas) = Tablero::obtener_configuracion(dificultad);
        let casillas = Tablero::bombas_mezcladas( ancho * largo, numero_bombas);

        let tablero = Tablero {
            ancho : ancho,
            largo : largo,
            casillas : casillas,
            es_ganador : false
        };
        
        return tablero;
    }

    // Aquí irían configuraciones adicionales a añadir
    pub fn obtener_configuracion(dificultad : Dificultad) -> (u16, u16, u16) {
        // Devuelve, en este orden: ancho y largo del tablero y número de bombas que contiene
        match dificultad {
            Dificultad::Facil => (8, 8, 10),
            Dificultad::Normal => (16, 16, 40),
            Dificultad::Dificil => (16, 30, 99)
        }

    }

    pub fn bombas_mezcladas(numero_casillas : u16, numero_bombas : u16) -> Vec<Casilla> {

        let mut casillas : Vec<Casilla> = vec![];

        for n in 0..numero_casillas{
            if n <= numero_bombas {
                casillas.push(Casilla::nueva(true));
            } else {
                casillas.push(Casilla::nueva(false));
            }
        }

        casillas.shuffle(&mut thread_rng());
        return casillas; 
        // Voy a usar los indices del vector
        // Calcular el número de cada casilla, ahora que las bombas están mezcladas

    }

    pub fn mostrar(&mut self) { 
        
        // Conversion de usize a u64  (en maquinas de 64 bits es lo mismo)

        for (casilla, i) in self.casillas.iter().zip(0u64..){
            
            print!("{} ", casilla.to_string());

            // Fin de la fila, salto de linea
            if i % u64::from(self.ancho) == 0 {
                println!("");
            }

        }
        
    }

    // Esta función es la encargada de actualizar el tablero con las nuevas
    // casillas que se le pasan. Lo más importante, es que desvela las casillas
    
    
    // El índice de las casillas a insertar, se
    // calcula a partir del índice de la central, dado que son contiguas.
    // el calculo para cada indice esta en utils, en casillas_contiguas()

    pub fn actualizar(nuevas_casillas : Vec<Casilla>, indice_central : u16) {
        
    }

}
