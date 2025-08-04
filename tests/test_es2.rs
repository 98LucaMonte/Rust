
#[cfg(test)]  
mod tests {
    use esercizi_esame_rust::es2::razionali::razionali::Razionali;

    #[test]
    fn test_new() {
        let raz = Razionali::new(3, 4);
        assert_eq!(raz.numeratore,3);
        assert_eq!(raz.denominatore,4);
    }

    #[test]
    fn test_numeratore_zero(){
        let raz = Razionali::new(0,5);
        assert_eq!(raz.numeratore,0);
        assert_eq!(raz.denominatore,1);
    }

    #[test]
    #[should_panic(expected = "Il denominatore deve essere diverso da 0")]
    fn test_denominatore_zero(){
        Razionali::new(5,0);
    }

    #[test]
    fn test_int_to_raz(){
        let raz = Razionali::int_to_raz(5);
        assert_eq!(raz.numeratore,5);
        assert_eq!(raz.denominatore,1);
    }

    #[test]
    fn test_somma(){
        let raz1 = Razionali::new(3,4);
        let raz2 = Razionali::new(5, 6);
        let raz_somma = raz1+raz2;
        assert_eq!(raz_somma.numeratore,19);
        assert_eq!(raz_somma.denominatore,12);
        
        let raz3 = Razionali::new(3,11);
        let raz4 = Razionali::new(-4, 3);
        let raz_somma1 = raz3 + raz4;
        assert_eq!(raz_somma1.numeratore,-35);
        assert_eq!(raz_somma1.denominatore,33);
        
        let raz5 = Razionali::new(5,22);
        let raz_somma2 = raz5 + 10;
        assert_eq!(raz_somma2.numeratore,225);
        assert_eq!(raz_somma2.denominatore,22);

        let raz6 = Razionali::new(5, 7);
        let raz_somma3 = raz6 + 0;
        assert_eq!(raz_somma3.numeratore,5);
        assert_eq!(raz_somma3.denominatore,7);
        
    }

    #[test]
    fn test_prodotto(){
        let raz1 = Razionali::new(3, 4);
        let raz2 = Razionali::new(5, 6);

        let prodotto = raz1*raz2;
        assert_eq!(prodotto.numeratore,5);
        assert_eq!(prodotto.denominatore,8);
        
        let raz3 = Razionali::new(3,11);
        let raz4 = Razionali::new(-4, 3);
        let raz_somma1 = raz3 * raz4;
        assert_eq!(raz_somma1.numeratore,-4);
        assert_eq!(raz_somma1.denominatore,11);
        
        let raz5 = Razionali::new(5,22);
        let raz_somma2 = raz5 * 10;
        assert_eq!(raz_somma2.numeratore,25);
        assert_eq!(raz_somma2.denominatore,11);

        let raz6 = Razionali::new(5, 7);
        let raz_somma3 = raz6 * 0;
        assert_eq!(raz_somma3.numeratore,0);
        assert_eq!(raz_somma3.denominatore,1);
    }

    #[test]
    fn test_riduzione_minimi_termini() {
        let mut raz = Razionali::new(12, 24);
        raz.riduzione_minimi_termini();
        assert_eq!(raz.numeratore,1);
        assert_eq!(raz.denominatore,2);

        let mut raz1 = Razionali::new(-55, 35);
        raz1.riduzione_minimi_termini();
        assert_eq!(raz1.numeratore,-11);
        assert_eq!(raz1.denominatore,7);

        let mut raz2 = Razionali::new(55, -35);
        raz2.riduzione_minimi_termini();
        assert_eq!(raz2.numeratore,11);
        assert_eq!(raz2.denominatore,-7);
    }

}