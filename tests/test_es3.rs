#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es3::{campo::Campo, cella::Cella, direzione::Direzione, giocatore::Giocatore, livello::Livello};

    #[test]
    fn test_campo_facile() {
        let campo: Campo = Campo::new(10, &Livello::Facile);
        
        let mut veleno = 0;
        let mut vuote = 0;
        let mut cibo = 0;
        let mut muro = 0;

        for riga in campo.mappa{
            for cella in riga {
                
                match cella {
                    Cella::Vuota(_) => vuote+=1,
                    Cella::Cibo(_) => cibo+=1,
                    Cella::Veleno(_) => veleno+=1,
                    Cella::Muro => muro+=1,
                    Cella::Player => (),
                }
            }
        }

        assert_eq!(muro,36);
        assert_eq!(vuote,45);
        assert_eq!(cibo,11);
        assert_eq!(veleno,8);

    }

    #[test]
    fn test_giocatore_facile(){
        let campo: Campo = Campo::new(9, &Livello::Facile);
        let giocatore: Giocatore = Giocatore::new(&campo, &Livello::Facile);

        assert_eq!(giocatore.forza,10);
        assert_eq!(giocatore.mosse,11);
    }

    #[test]
    fn test_campo_medio() {
        let campo: Campo = Campo::new(8, &Livello::Medio);
        
        let mut veleno = 0;
        let mut vuote = 0;
        let mut cibo = 0;
        let mut muro = 0;

        for riga in campo.mappa{
            for cella in riga {
                
                match cella {
                    Cella::Vuota(_) => vuote+=1,
                    Cella::Cibo(_) => cibo+=1,
                    Cella::Veleno(_) => veleno+=1,
                    Cella::Muro => muro+=1,
                    Cella::Player => (),
                }
            }
        }

        assert_eq!(muro,28);
        assert_eq!(vuote,20);
        assert_eq!(cibo,8);
        assert_eq!(veleno,8);

    }

    #[test]
    fn test_giocatore_medio() {
        let campo: Campo = Campo::new(8, &Livello::Medio);
        let giocatore: Giocatore = Giocatore::new(&campo, &Livello::Medio);

        assert_eq!(giocatore.forza,8);
        assert_eq!(giocatore.mosse,8);
    }

    #[test]
    fn test_campo_difficile() {
        let campo: Campo = Campo::new(7, &Livello::Difficile);
        
        let mut veleno = 0;
        let mut vuote = 0;
        let mut cibo = 0;
        let mut muro = 0;

        for riga in campo.mappa{
            for cella in riga {
                
                match cella {
                    Cella::Vuota(_) => vuote+=1,
                    Cella::Cibo(_) => cibo+=1,
                    Cella::Veleno(_) => veleno+=1,
                    Cella::Muro => muro+=1,
                    Cella::Player => (),
                }
            }
        }

        assert_eq!(muro,24);
        assert_eq!(vuote,10);
        assert_eq!(cibo,6);
        assert_eq!(veleno,9);

    }

    #[test]
    fn test_giocatore_difficile() {
        let campo: Campo = Campo::new(7, &Livello::Difficile);
        let giocatore: Giocatore = Giocatore::new(&campo, &Livello::Difficile);

        assert_eq!(giocatore.forza,5);
        assert_eq!(giocatore.mosse,7);
    }

    #[test]
    fn test_modifica_forza_giocatore() {

        let campo: Campo = Campo::new(7, &Livello::Difficile);
        let mut giocatore: Giocatore = Giocatore::new(&campo, &Livello::Difficile);
        
        for riga in 0..campo.mappa.len(){
            for colonna in 0..campo.mappa.len(){
                match campo.mappa[riga][colonna]{
                    Cella::Veleno(_) => giocatore.posizione = (riga,colonna),
                    Cella::Cibo(_) |  Cella::Muro |  Cella::Vuota(_) | Cella::Player => (),
                }
            }
        }

        giocatore.modifica_forza_giocatore(&campo);
        assert_eq!(giocatore.forza,1);

        for riga in 0..campo.mappa.len(){
            for colonna in 0..campo.mappa.len(){
                match campo.mappa[riga][colonna]{
                    Cella::Cibo(_) => giocatore.posizione = (riga,colonna),
                    Cella::Veleno(_) |  Cella::Muro |  Cella::Vuota(_) | Cella::Player => (),
                }
            }
        }

        giocatore.modifica_forza_giocatore(&campo);
        assert_eq!(giocatore.forza,2);

    }

    #[test]
    fn test_controllo_muro() {
        
        let campo: Campo = Campo::new(7, &Livello::Difficile);

        let mut info: (Direzione,usize,usize) = campo.controllo_muro(Direzione::Su, (0,1));
        assert_eq!(info.0,Direzione::Giu);
        assert_eq!((info.1,info.2),(2,1));
        info = campo.controllo_muro(Direzione::Su, (0,2));
        assert_eq!(info.0,Direzione::Giu);
        assert_eq!((info.1,info.2),(2,2));
        info = campo.controllo_muro(Direzione::Su, (0,3));
        assert_eq!(info.0,Direzione::Giu);
        assert_eq!((info.1,info.2),(2,3));
        info = campo.controllo_muro(Direzione::Su, (0,4));
        assert_eq!(info.0,Direzione::Giu);
        assert_eq!((info.1,info.2),(2,4));
        info = campo.controllo_muro(Direzione::Su, (0,5));
        assert_eq!(info.0,Direzione::Giu);
        assert_eq!((info.1,info.2),(2,5));

        info = campo.controllo_muro(Direzione::Giu, (6,1));
        assert_eq!(info.0,Direzione::Su);
        assert_eq!((info.1,info.2),(4,1));
        info = campo.controllo_muro(Direzione::Giu, (6,2));
        assert_eq!(info.0,Direzione::Su);
        assert_eq!((info.1,info.2),(4,2));
        info = campo.controllo_muro(Direzione::Giu, (6,3));
        assert_eq!(info.0,Direzione::Su);
        assert_eq!((info.1,info.2),(4,3));
        info = campo.controllo_muro(Direzione::Giu, (6,4));
        assert_eq!(info.0,Direzione::Su);
        assert_eq!((info.1,info.2),(4,4));
        info = campo.controllo_muro(Direzione::Giu, (6,5));
        assert_eq!(info.0,Direzione::Su);
        assert_eq!((info.1,info.2),(4,5));

        info = campo.controllo_muro(Direzione::Sinistra, (1,0));
        assert_eq!(info.0,Direzione::Destra);
        assert_eq!((info.1,info.2),(1,2));
        info = campo.controllo_muro(Direzione::Sinistra, (2,0));
        assert_eq!(info.0,Direzione::Destra);
        assert_eq!((info.1,info.2),(2,2));
        info = campo.controllo_muro(Direzione::Sinistra, (3,0));
        assert_eq!(info.0,Direzione::Destra);
        assert_eq!((info.1,info.2),(3,2));
        info = campo.controllo_muro(Direzione::Sinistra, (4,0));
        assert_eq!(info.0,Direzione::Destra);
        assert_eq!((info.1,info.2),(4,2));
        info = campo.controllo_muro(Direzione::Sinistra, (5,0));
        assert_eq!(info.0,Direzione::Destra);
        assert_eq!((info.1,info.2),(5,2));

        info = campo.controllo_muro(Direzione::Destra, (1,6));
        assert_eq!(info.0,Direzione::Sinistra);
        assert_eq!((info.1,info.2),(1,4));
        info = campo.controllo_muro(Direzione::Destra, (2,6));
        assert_eq!(info.0,Direzione::Sinistra);
        assert_eq!((info.1,info.2),(2,4));
        info = campo.controllo_muro(Direzione::Destra, (3,6));
        assert_eq!(info.0,Direzione::Sinistra);
        assert_eq!((info.1,info.2),(3,4));
        info = campo.controllo_muro(Direzione::Destra, (4,6));
        assert_eq!(info.0,Direzione::Sinistra);
        assert_eq!((info.1,info.2),(4,4));
        info = campo.controllo_muro(Direzione::Destra, (5,6));
        assert_eq!(info.0,Direzione::Sinistra);
        assert_eq!((info.1,info.2),(5,4));

    }

    #[test]
    fn test_cambia_valore_cella() {

        let mut campo: Campo = Campo::new(7, &Livello::Difficile);
        let mut posizione_veleno: (usize,usize) = (0,0);
        let mut posizione_cibo: (usize,usize) = (0,0);
        let mut posizione_vuota: (usize,usize) = (0,0);
        let mut posizione_giocatore: (usize,usize) = (0,0);
        let posizione_appoggio:(usize,usize) = (0,0);

        for riga in 0..campo.mappa.len(){
            for colonna in 0..campo.mappa.len(){
                match campo.mappa[riga][colonna]{
                    Cella::Veleno(_) => posizione_veleno = (riga,colonna),
                    Cella::Cibo(_) |  Cella::Muro |  Cella::Vuota(_) | Cella::Player => (),
                }
            }
        }

        campo.cambia_valore_cella(posizione_veleno, posizione_appoggio);
        assert_eq!(campo.mappa[posizione_veleno.0][posizione_veleno.1],Cella::Player);

        for riga in 0..campo.mappa.len(){
            for colonna in 0..campo.mappa.len(){
                match campo.mappa[riga][colonna]{
                    Cella::Cibo(_) => posizione_cibo = (riga,colonna),
                    Cella::Veleno(_) |  Cella::Muro |  Cella::Vuota(_) | Cella::Player => (),
                }
            }
        }

        campo.cambia_valore_cella(posizione_cibo, posizione_appoggio);
        assert_eq!(campo.mappa[posizione_cibo.0][posizione_cibo.1],Cella::Player);

        for riga in 0..campo.mappa.len(){
            for colonna in 0..campo.mappa.len(){
                match campo.mappa[riga][colonna]{
                    Cella::Vuota(_) => posizione_vuota = (riga,colonna),
                    Cella::Veleno(_) |  Cella::Muro |  Cella::Cibo(_) | Cella::Player => (),
                }
            }
        }

        campo.cambia_valore_cella(posizione_vuota, posizione_appoggio);
        assert_eq!(campo.mappa[posizione_vuota.0][posizione_vuota.1],Cella::Player);

        for riga in 0..campo.mappa.len(){
            for colonna in 0..campo.mappa.len(){
                match campo.mappa[riga][colonna]{
                    Cella::Player => posizione_giocatore = (riga,colonna),
                    Cella::Veleno(_) |  Cella::Muro |  Cella::Cibo(_) | Cella::Vuota(_) => (),
                }
            }
        }

        campo.cambia_valore_cella(posizione_appoggio, posizione_giocatore);
        assert_eq!(campo.mappa[posizione_giocatore.0][posizione_giocatore.1],Cella::Vuota(0));

    }
}