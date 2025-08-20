
#[cfg(test)]  
mod tests {
    use esercizi_esame_rust::es2::razionali::Razionali;
    
    /** 
     * Test utile per verificare la corretta creazione di un razionale.
    */
    #[test]
    fn test_new() {
        let raz = Razionali::new(3, 4);
        assert_eq!(raz.get_numeratore(),3);
        assert_eq!(raz.get_denominatore(),4);
    }

    /** 
     * Test utile per verificare la corretta creazione di un razionale quando il numeratore è 0.
    */
    #[test]
    fn test_get_numeratore_zero(){
        let raz = Razionali::new(0,5);
        assert_eq!(raz.get_numeratore(),0);
        assert_eq!(raz.get_denominatore(),1);
    }

    /** 
     * Test utile per verificare la corretta creazione di un razionale quando il denominatore è 0.
     * Questo test panica quindi serve il should_panic per gestirlo correttamente.
    */
    #[test]
    #[should_panic(expected = "Il denominatore deve essere diverso da 0")]
    fn test_denominatore_zero(){
        Razionali::new(5,0);
    }

    /** 
     * Test utile per verificare la corretta conversione di un intero in un razionale.
    */
    #[test]
    fn test_int_to_raz(){
        let raz = Razionali::int_to_raz(5);
        assert_eq!(raz.get_numeratore(),5);
        assert_eq!(raz.get_denominatore(),1);
    }

    /** 
     * Test utile per verificare la correttezza della somma tra razionali e tra razionale e intero.
    */
    #[test]
    fn test_somma(){
        let raz1 = Razionali::new(3,4);
        let raz2 = Razionali::new(5, 6);
        let raz10 = Razionali::new(19, 12);
        let raz_somma = raz1+raz2;
        assert_eq!(raz_somma.get_numeratore(),19);
        assert_eq!(raz_somma.get_denominatore(),12);
        assert!(raz10 == raz_somma);
        
        let raz3 = Razionali::new(3,11);
        let raz4 = Razionali::new(-4, 3);
        let raz11 = Razionali::new(-35, 33);
        let raz_somma1 = raz3 + raz4;
        assert_eq!(raz_somma1.get_numeratore(),-35);
        assert_eq!(raz_somma1.get_denominatore(),33);
        assert!(raz11 == raz_somma1);
        
        let raz5 = Razionali::new(5,22);
        let raz12 = Razionali::new(225, 22);
        let raz_somma2 = raz5 + 10;
        assert_eq!(raz_somma2.get_numeratore(),225);
        assert_eq!(raz_somma2.get_denominatore(),22);
        assert!(raz12 == raz_somma2);

        let raz6 = Razionali::new(5, 7);
        let raz13 = Razionali::new(5, 7);
        let raz_somma3 = raz6 + 0;
        assert_eq!(raz_somma3.get_numeratore(),5);
        assert_eq!(raz_somma3.get_denominatore(),7);
        assert!(raz13 == raz_somma3);
    }

    /** 
     * Test utile per verificare la correttezza del prodotto tra razionali e tra razionale e intero.
    */
    #[test]
    fn test_prodotto(){
        let raz1 = Razionali::new(3, 4);
        let raz2 = Razionali::new(5, 6);
        let raz21 = Razionali::new(5, 8);
        let prodotto = raz1*raz2;
        assert_eq!(prodotto.get_numeratore(),5);
        assert_eq!(prodotto.get_denominatore(),8);
        assert!(raz21 == prodotto);
        
        let raz3 = Razionali::new(3,11);
        let raz4 = Razionali::new(-4, 3);
        let raz22 = Razionali::new(-4, 11);
        let prodotto1 = raz3 * raz4;
        assert_eq!(prodotto1.get_numeratore(),-4);
        assert_eq!(prodotto1.get_denominatore(),11);
        assert!(raz22 == prodotto1);
        
        let raz5 = Razionali::new(5,22);
        let raz23 = Razionali::new(25, 11);
        let prodotto2 = raz5 * 10;
        assert_eq!(prodotto2.get_numeratore(),25);
        assert_eq!(prodotto2.get_denominatore(),11);
        assert!(raz23 == prodotto2);

        let raz6 = Razionali::new(5, 7);
        let raz24 = Razionali::new(0, 1);
        let prodotto3 = raz6 * 0;
        assert_eq!(prodotto3.get_numeratore(),0);
        assert_eq!(prodotto3.get_denominatore(),1);
        assert!(raz24 == prodotto3);
    }

    /** 
     * Test utile per verificare la correttezza della riduzione in minimi termini
    */
    #[test]
    fn test_riduzione_minimi_termini() {
        let mut raz = Razionali::new(12, 24);
        raz.riduzione_minimi_termini();
        assert_eq!(raz.get_numeratore(),1);
        assert_eq!(raz.get_denominatore(),2);

        let mut raz1 = Razionali::new(-55, 35);
        raz1.riduzione_minimi_termini();
        assert_eq!(raz1.get_numeratore(),-11);
        assert_eq!(raz1.get_denominatore(),7);

        let mut raz2 = Razionali::new(55, -35);
        raz2.riduzione_minimi_termini();
        assert_eq!(raz2.get_numeratore(),11);
        assert_eq!(raz2.get_denominatore(),-7);
    }

}