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
}

impl fmt::Display for Casilla {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let imprimir;

        if (self.revelada) {
            imprimir = match self.contenido {
                Contenido::None => " ",
                Contenido::Bomba => "X",
                Contenido::Number(x) => &x.to_string(),
            };
        } else {
            //Casilla oculta
            imprimir = "@"
        }

        write!(f, "{}", imprimir)
    }
}
