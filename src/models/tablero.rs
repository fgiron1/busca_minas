use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fmt::{self, Display};
use std::io::{self, Write};


use crate::models::casilla::Casilla;

#[derive(Debug, PartialEq)]
pub enum Dificultad {
    Facil,
    Normal,
    Dificil,
}

#[derive(Debug)]
pub struct InputError;

impl Dificultad {
    pub fn from_str(s: String) -> Result<Dificultad, InputError> {
        return match s.as_str() {
            "fácil" => Ok(Dificultad::Facil),
            "normal" => Ok(Dificultad::Normal),
            "dificil" => Ok(Dificultad::Dificil),
            _ => Err(InputError),
        };
    }
}

pub struct Tablero {
    pub ancho: u16,
    pub largo: u16,
    pub casillas: Vec<Casilla>,
    pub es_ganador: bool,
}

impl Tablero {
    pub fn construir_tablero(dificultad: Dificultad) -> Tablero {
        let (ancho, largo, numero_bombas) = Tablero::parsear_configuracion(dificultad);
        let casillas = Tablero::bombas_mezcladas(ancho * largo, numero_bombas);

        let tablero = Tablero {
            ancho,
            largo,
            casillas,
            es_ganador: false,
        };

        return tablero;
    }

    // Aquí irían configuraciones adicionales a añadir
    pub fn parsear_configuracion(dificultad: Dificultad) -> (u16, u16, u16) {
        // Devuelve, en este orden: ancho y largo del tablero y número de bombas que contiene
        match dificultad {
            Dificultad::Facil => (8, 8, 10),
            Dificultad::Normal => (16, 16, 40),
            Dificultad::Dificil => (16, 30, 99),
        }
    }

    pub fn bombas_mezcladas(numero_casillas: u16, numero_bombas: u16) -> Vec<Casilla> {
        let mut casillas: Vec<Casilla> = vec![];

        for n in 0..numero_casillas {
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

    pub fn mostrar(&self) {
        // Conversion de usize a u64  (en maquinas de 64 bits es lo mismo)

        for (casilla, i) in self.casillas.iter().zip(0u64..) {
            
            io::stdout().flush().unwrap();
            print!("{} ", casilla);
            
            // Fin de la fila, salto de linea
            if i % u64::from(self.ancho) == 0 {
                io::stdout().flush().unwrap();
                println!("");
            }
        }
    }

    /*
        Esta función es la encargada de actualizar el tablero con las nuevas
        casillas que se le pasan. Lo más importante, es que desvela las casillas

        TODO : Definir bien las fases de actualizar el tablero
        TODO : Imprimir numeros al lado de filas y columnas
    */

    pub fn actualizar(&mut self, nuevas_casillas: Vec<Casilla>, indice_central: u64) {
        
        let mut indices_contiguos : Vec<u64> = Casilla::casillas_contiguas(indice_central);

        
        
    }
}
