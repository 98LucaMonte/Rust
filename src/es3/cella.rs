

use std::fmt;  

#[derive(Clone,PartialEq,Debug)]
pub enum Cella{
    Vuota(i32),
    Cibo(i32),
    Veleno(i32),
    Muro,
    Player,
}

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
