#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es5::{conto_bancario::ContoBancario, stato::StatoTipo};

    #[test]
    fn test_conto_rosso() {
        let conto_bancario_luca = ContoBancario::new("Luca".to_string(), 100.0, 
                                                                    1000.0, 10000.0, 0.05);

        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Rosso);
        assert_eq!(conto_bancario_luca.saldo, 100.0);
    }

    #[test]
    fn test_conto_rosso_deposita_rosso() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 100.0, 
                                                                    1000.0, 10000.0, 0.05);

        conto_bancario_luca.stato.deposita(& mut conto_bancario_luca.saldo, 100.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(),StatoTipo::Rosso);
        assert_eq!(conto_bancario_luca.saldo, 200.0);
        conto_bancario_luca.stato.deposita(& mut conto_bancario_luca.saldo, 1000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Argento);
        assert_eq!(conto_bancario_luca.saldo, 1200.0);
        conto_bancario_luca.stato.deposita(& mut conto_bancario_luca.saldo, 10000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Oro);
        assert_eq!(conto_bancario_luca.saldo, 11200.0);
    }
    
    #[test]
    fn test_conto_argento() {
        let conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Argento);
        assert_eq!(conto_bancario_luca.saldo, 3000.0);
    }
 
    #[test]
    fn test_conto_argento_deposita() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);

        conto_bancario_luca.stato.deposita(& mut conto_bancario_luca.saldo, 2000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Argento);
        assert_eq!(conto_bancario_luca.saldo, 5000.0);
        conto_bancario_luca.stato.deposita(& mut conto_bancario_luca.saldo, 30000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Oro);
        assert_eq!(conto_bancario_luca.saldo, 35000.0);
    }
 
    #[test]
    fn test_conto_argento_preleva() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 3000.0, 
                                                                    1000.0, 10000.0, 0.05);

        conto_bancario_luca.stato.preleva(& mut conto_bancario_luca.saldo, 1000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Argento);
        assert_eq!(conto_bancario_luca.saldo, 2000.0);
        conto_bancario_luca.stato.preleva(& mut conto_bancario_luca.saldo, 1200.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Rosso);
        assert_eq!(conto_bancario_luca.saldo, 800.0);
    }

    #[test]
    fn test_conto_oro() {
        let conto_bancario_luca = ContoBancario::new("Luca".to_string(), 100000.0, 
                                                                    1000.0, 10000.0, 0.05);
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Oro);
        assert_eq!(conto_bancario_luca.saldo, 100000.0);
    }

    #[test]
    fn test_conto_oro_deposita() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);

        
        conto_bancario_luca.stato.deposita(& mut conto_bancario_luca.saldo, 30000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Oro);
        assert_eq!(conto_bancario_luca.saldo, 42000.0);
    }

    #[test]
    fn test_conto_oro_preleva() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);

        
        conto_bancario_luca.stato.preleva(& mut conto_bancario_luca.saldo, 300.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Oro);
        assert_eq!(conto_bancario_luca.saldo, 11700.0);
        conto_bancario_luca.stato.preleva(& mut conto_bancario_luca.saldo, 3000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Argento);
        assert_eq!(conto_bancario_luca.saldo, 8700.0);
        conto_bancario_luca.stato.preleva(& mut conto_bancario_luca.saldo, 8000.0);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Rosso);
        assert_eq!(conto_bancario_luca.saldo, 700.0);
    }

    #[test]
    fn test_conto_oro_paga_interessi() {
        let mut conto_bancario_luca = ContoBancario::new("Luca".to_string(), 12000.0, 
                                                                    1000.0, 10000.0, 0.05);

        conto_bancario_luca.stato.paga_interessi(& mut conto_bancario_luca.saldo,&conto_bancario_luca.interesse);
        conto_bancario_luca.aggiorna_stato_conto();
        assert_eq!(conto_bancario_luca.stato.stato_attuale_conto(), StatoTipo::Oro);
        assert_eq!(conto_bancario_luca.saldo, 12600.0);
    }

}