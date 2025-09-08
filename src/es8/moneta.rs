use std::fmt;
use rand::Rng;

/**
 * Questo è un enum che rappresenta la moneta da lanciare.
 */
pub enum Moneta{  
    Testa,
    Croce
}

/** 
 * Questo è il trait Display utile per avere una visualizzazione delle informazioni sulla moneta.
*/
impl fmt::Display for Moneta {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Moneta::Testa =>{
                write!(f," Testa")
            }
            Moneta::Croce =>{
                write!(f," Croce")
            }
        }
    }
}

impl Moneta {
    /** 
     * Funzione utile a fare un lancio randomico di testa o croce.
    */
    pub fn lancio_moneta()-> Moneta{
        let num = rand::thread_rng().gen_range(0..=1);

        match num {
            0 => {
                return Moneta::Croce
            },
            _ => {
                return Moneta::Testa
            }
        };
    }
}
