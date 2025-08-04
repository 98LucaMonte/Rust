use std::fmt;

pub enum Livello {
    Facile,Medio,Difficile
}

/** 
 *  Con tale metodo vado ad indicare come voglio stampare le informazioni sul Livello.
 */
impl fmt::Display for Livello{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let livello = match self {
            Livello::Facile =>{"Facile"},
            Livello::Medio =>{"Medio"},
            Livello::Difficile =>{"Difficile"},
        };
        write!(f, "{}", livello)?;
        Ok(())
    }
} 
 
impl Livello{
    /** 
     *  Con tale metodo si ritorna una tupla che a seconda del livello scelto restituisce nelle seguenti posizioni:
     *  0. -> forza del giocatore
     *  1. -> cibo all'interno della mappa
     *  2. -> veleno all'interno della mappa
     *  3. -> mosse assegnate al giocatore
     */
    pub fn scelta_livello(&self,n:u32) -> (i32,u32,u32,u32){
        match self{
            Livello::Facile=>{
                let forza:i32 = 1 + n as i32;
                let cibo:u32 = n + 1;
                let veleno:u32 = n - 2;
                let mosse:u32 = n + 2;
                (forza,cibo,veleno,mosse)
            }
            Livello::Medio=>{
                let forza:i32 = n as i32;
                let cibo:u32 = n;
                let veleno:u32 = n;
                let mosse:u32 = n;
                (forza,cibo,veleno,mosse)
            }
            Livello::Difficile=>{
                let forza:i32 = (n as i32) - 2;
                let cibo:u32 = n - 1;
                let veleno:u32 = n + 2;
                let mosse:u32 = n;
                (forza,cibo,veleno,mosse)
            },
        }
    }

}

