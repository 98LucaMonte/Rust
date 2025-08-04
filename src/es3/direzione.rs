 
 
use std::fmt;

#[derive(Clone,PartialEq,Debug)]
pub enum Direzione {
    Su,Giu,Destra,Sinistra
}

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

