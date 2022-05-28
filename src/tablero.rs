use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Debug)]
pub enum Dificultad {
    Facil,
    Normal,
    Dificil
}

#[derive(Debug)]
pub struct InputError;

impl Dificultad {
    fn from_str(s: &str) -> Result<Dificultad, InputError> {
        return match s {
            "facil" => Ok(Dificultad::Facil),
            "normal" => Ok(Dificultad::Normal),
            "dificil" => Ok(Dificultad::Dificil),
            _ => Err(InputError)
            
        };
    }
}

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

pub struct Tablero {
    ancho: u16,
    largo: u16,
    casillas: Vec<Casilla>,
    es_ganador: bool,
}

impl Tablero {

    pub fn construir_tablero(dificultad : Dificultad) -> Tablero {
        
        let (ancho, largo, numero_bombas) = Tablero::obtener_configuracion(dificultad);

        let mut casillas : Vec<Casilla> = Tablero::crear_casillas(ancho * largo, numero_bombas);

        let mut tablero = Tablero {
            ancho : ancho,
            largo : largo,
            casillas : casillas,
            es_ganador : false
        };
        
        return tablero;
    }

    pub fn obtener_configuracion(dificultad : Dificultad) -> (u16, u16, u16) {
        // Devuelve, en este orden: ancho y largo del tablero y número de bombas que contiene
        match dificultad {
            Dificultad::Facil => (8, 8, 10),
            Dificultad::Normal => (16, 16, 40),
            Dificultad::Dificil => (16, 30, 99)
        }

    }

    fn crear_casillas(numero_casillas : u16, numero_bombas : u16) -> Vec<Casilla> {

        let mut casillas : Vec<Casilla> = vec![];

        for n in 0..numero_casillas{
            
            if n <= numero_bombas {
                casillas.push(Casilla::nueva(true));
            }

            casillas.push(Casilla::nueva(false));
        }

        casillas.shuffle(&mut thread_rng());        
        // Voy a usar los indices del vector

        return casillas;
    }
}
