use std::fmt;

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

    
    pub fn nueva(bomba: bool) -> Casilla {
        let casilla: Casilla = Casilla {
            contenido: if bomba {
                Contenido::Bomba
            } else {
                Contenido::None
            },
            revelada: false,
        };

        return casilla;
    }

    /*
        Esta función comprueba si una casilla se encuentra en el borde del tablero o no.
        Extrae a partir del tamaño

        // Los indices del borde de arriba son del 0 hasta el (ancho)
        // Los índices de los bordes laterales son del (ancho) * número de fila que sea
        // (El izquierdo y el derecho se diferencian porque el derecho siempre tiene una fila más)
        // Los indices del borde de abajo son del 0 + largo hasta el (ancho + largo)
    */
    pub fn es_borde(indice_candidato: u32, (ancho, largo): (u32, u32)) -> bool {
        let mut indices_invalidos: Vec<u32> = vec![];

        let mut borde_izquierdo: Vec<u32> = vec![];
        let mut borde_derecho: Vec<u32> = vec![];

        // Borde superior
        indices_invalidos.append(&mut (0..ancho).collect());

        // Borde inferior
        indices_invalidos.append(&mut (largo..(ancho + largo)).collect());

        // Bordes izquierdo y derecho

        for n in 0..largo {
            borde_izquierdo.push((ancho * n) - 1);
            borde_derecho.push(ancho * (n + 1) - 1);
        }

        indices_invalidos.append(&mut borde_izquierdo);
        indices_invalidos.append(&mut borde_derecho);

        return indices_invalidos.contains(&indice_candidato);
    }

    /*
        Esta función calcula el índice de las casillas contiguas a una dada, segun su índice.

        En el momento que se selecciona una casilla se comprueban
        las adyacentes y, no antes.

        1   2   3   --> Orden en el que se almacenan los índices
        4   X   5   --> adyacentes al índice de la casilla "X" en el
        6   7   8   --> vector resultado de índices.

        El resultado siempre contendrá algunos índices erróneos si la casilla se encuentra
        en uno de los bordes del tablero.

    */
    pub fn casillas_contiguas(indice: u64) -> Vec<u64> {
        // Quitarle 7 es subir una fila
        // Sumarle 7 es bajar una fila

        vec![
            indice - 9,
            indice - 8,
            indice - 7,
            indice - 1,
            indice + 1,
            indice + 7,
            indice + 8,
            indice + 9,
        ]
    }
}

impl fmt::Display for Casilla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let imprimir: String;

        if self.revelada {
            imprimir = match self.contenido {
                Contenido::None => String::from(" "),
                Contenido::Bomba => String::from("X"),
                Contenido::Number(x) => x.to_string(),
            };
        } else {
            //Casilla oculta
            imprimir = String::from("@");
        }

        write!(f, "{}", imprimir)
    }
}
