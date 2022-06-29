use crate::models::casilla::Casilla;


 /*
 
 Calcular numeros tiene que ser llamado cuando el usuraio introduzca
 la posición de la bomba a desvelar. Si era una bomba, explota y pierde
 si no, mira lo que hay a su inmediato alrededor, y si no hay ninguna bomba,
 libera todo el tablero hasta que alguna casilla de las desbloqueadas tenga algo
 adyacente, en cuyo caso lo revela y para
 
 */

pub fn numeros_contiguos(casillas: Vec<Casilla>) -> Vec<Casilla> {
    for (casilla, i) in casillas.iter().zip(0u64..) {
        casillas_contiguas(i);
    }

    return casillas;
}



