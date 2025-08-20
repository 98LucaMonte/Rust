use std::fmt;
/**
 * Questo è un enum che rappresenta le diverse Direzioni che può prendere il giocatore.
 */
#[derive(Clone,PartialEq,Debug)]
pub enum Direzione {
    Su,Giu,Destra,Sinistra
}

/** 
 * Questo è il trait Display utile per stampare la Direzione.
*/
impl fmt::Display for Direzione {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Direzione::Su =>{
                write!(f," Su")
            }
            Direzione::Giu =>{
                write!(f," Giu")
            }
            Direzione::Destra =>{
                write!(f," Destra")
            }
            Direzione::Sinistra =>{
                write!(f," Sinistra")
            }
        }
    }
}

