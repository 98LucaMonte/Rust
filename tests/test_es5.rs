#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es5::{conto_bancario::ContoBancario};

    /**
     * Test utile se il costruttore di conto bancario panica se il limite inferiore è maggiore del limite superiore.
     */
    #[test]
    #[should_panic(expected = "Il limite superiore deve essere maggiore del limite inferiore")]
    fn test_limiti_errati(){
        ContoBancario::new("Luca".to_string(), 100.0,1000000.0, 10000.0, 0.05);
    }

    /**
     * Test utile se il conto bancario creato con tali parametri è nello Stato Rosso e se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_rosso() {
        let conto_bancario_luca = ContoBancario::new("Luca".to_string(), 100.0, 
                                                                    1000.0, 10000.0, 0.05);

        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(),"Rosso");
        assert_eq!(conto_bancario_luca.get_saldo(), 100.0);
    }

    /**
     * Test utile se il conto bancario quando è nello Stato Rosso aggiorna il suo stato dopo che viene eseguito il metodo deposita.
     * Il test verifica se aumentando il saldo del conto si passa da Stato Rosso a Stato Argento e poi da Stato Argento a Stato Oro.
     * Il test verifica anche se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_rosso_deposita_rosso() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 100.0, 
                                                                    1000.0, 10000.0, 0.05);

        conto_bancario_luca.deposita(100.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(),"Rosso");
        assert_eq!(conto_bancario_luca.get_saldo(), 200.0);

        conto_bancario_luca.deposita( 1000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Argento");
        assert_eq!(conto_bancario_luca.get_saldo(), 1200.0);

        conto_bancario_luca.deposita( 10000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 11200.0);
    }
    
    /**
     * Test utile se il conto bancario creato con tali parametri è nello Stato Argento e se il saldo restituito corrisponde.
     */
     #[test]
    fn test_conto_argento() {
        let conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Argento");
        assert_eq!(conto_bancario_luca.get_saldo(), 3000.0);
    }
    
    /**
     * Test utile se il conto bancario quando è nello Stato Argento aggiorna il suo stato dopo che viene eseguito il metodo deposita.
     * Il test verifica se aumentando il saldo del conto si passa da Stato Argento a Stato Oro.
     * Il test verifica anche se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_argento_deposita() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);

        conto_bancario_luca.deposita(2000.0);        
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Argento");
        assert_eq!(conto_bancario_luca.get_saldo(), 5000.0);

        conto_bancario_luca.deposita(30000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 35000.0);
    }
    
    /**
     * Test utile se il conto bancario quando è nello Stato Argento aggiorna il suo stato dopo che viene eseguito il metodo preleva.
     * Il test verifica se diminuendo il saldo del conto si passa da Stato Argento a Stato Rosso.
     * Il test verifica anche se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_argento_preleva() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);
        conto_bancario_luca.preleva(1000.0);        
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Argento");
        assert_eq!(conto_bancario_luca.get_saldo(), 2000.0);
        
        conto_bancario_luca.preleva(1200.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Rosso");
        assert_eq!(conto_bancario_luca.get_saldo(), 800.0);
    }

    /**
     * Test utile se il conto bancario quando è nello Stato Argento aggiorna il suo stato dopo che viene eseguito il metodo preleva.
     * Rispetto al test precedente si vuole verificare se si preleva anche il saldo è inferiore rispetto alla quantità richiesta da prelevare.
    */
    #[test]
    fn test_conto_argento_preleva_massimo_importo() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);
        conto_bancario_luca.preleva(3000.1);        
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Argento");
        assert_eq!(conto_bancario_luca.get_saldo(), 3000.0);
        
        conto_bancario_luca.preleva(3000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Rosso");
        assert_eq!(conto_bancario_luca.get_saldo(), 0.0);
    }

    /**
     * Test utile se il conto bancario creato con tali parametri è nello Stato Oro e se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_oro() {
        let conto_bancario_luca = ContoBancario::new("Luca".to_string(), 100000.0, 
                                                                    1000.0, 10000.0, 0.05);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 100000.0);
    }

    /**
     * Test utile se il conto bancario quando è nello Stato Oro aggiorna il suo Stato dopo che viene eseguito il metodo deposita.
     * Il test verifica se aumentando il saldo del conto si rimane effettivamente nello Stato Oro.
     * Il test verifica anche se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_oro_deposita() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);
        conto_bancario_luca.deposita(30000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 42000.0);
    }

    /**
     * Test utile se il conto bancario quando è nello Stato Oro aggiorna il suo stato dopo che viene eseguito il metodo preleva.
     * Il test verifica se diminuendo il saldo del conto si passa da Stato Oro a Stato Argento e da Stato Argento a Stato Rosso.
     * Il test verifica anche se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_oro_preleva() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);
        conto_bancario_luca.preleva(300.0);        
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 11700.0);

        conto_bancario_luca.preleva(3000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Argento");
        assert_eq!(conto_bancario_luca.get_saldo(), 8700.0);

        conto_bancario_luca.preleva(8000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Rosso");
        assert_eq!(conto_bancario_luca.get_saldo(), 700.0);
    }

     /**
     * Test utile se il conto bancario quando è nello Stato Oro aggiorna il suo stato dopo che viene eseguito il metodo preleva.
     * Rispetto al test precedente si vuole verificare se si preleva anche il saldo è inferiore rispetto alla quantità richiesta da prelevare.
    */
    #[test]
    fn test_conto_oro_preleva_massimo_importo() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);
        conto_bancario_luca.preleva(12000.1);        
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 12000.0);

        conto_bancario_luca.preleva(12000.0);
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Rosso");
        assert_eq!(conto_bancario_luca.get_saldo(), 0.0);

    }

    /**
     * Test utile se il conto bancario quando è nello Stato Oro aggiorna il suo stato dopo che viene eseguito il metodo paga interessi.
     * Il test verifica se aumentando il saldo del conto si rimane effettivamente nello Stato Oro.
     * Il test verifica anche se il saldo restituito corrisponde.
     */
    #[test]
    fn test_conto_oro_paga_interessi() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);
        conto_bancario_luca.paga_interessi();
        assert_eq!(conto_bancario_luca.get_stato().stato_attuale_conto(), "Oro");
        assert_eq!(conto_bancario_luca.get_saldo(), 12600.0);
    }

}