

use std::fmt;
use rand::Rng;
pub enum Moneta{  
    Testa,
    Croce
}

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
    
    pub fn lancio_moneta()-> Moneta{
        println!("Lancio della moneta....");
        let num = rand::thread_rng().gen_range(0..=1);

        match num {
            0 => return Moneta::Croce,
            _ => return Moneta::Testa,
        };
    }
}
