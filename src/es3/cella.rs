use std::fmt;  
/**
 * Questo è un enum che rappresenta le tipologie di Celle che ci possono essere all'interno di un campo da gioco.
 */
#[derive(Clone,PartialEq,Debug)]
pub enum Cella{
    Vuota(i32),
    Cibo(i32),
    Veleno(i32),
    Muro,
    Player,
}

/** 
 * Questo è il trait Display utile per avere una visualizzazione delle Celle.
*/
impl fmt::Display for Cella {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Cella::Cibo(..) =>{
                write!(f," 🍎")
            }
            Cella::Veleno(..) =>{
                write!(f," 🧪")
            }
            Cella::Vuota(..) =>{
                write!(f," ⚪")
            }
            Cella::Muro => {
                write!(f," 🧱")
            }
            Cella::Player => {
                write!(f," 🔵")
            }
        }
    }
}
