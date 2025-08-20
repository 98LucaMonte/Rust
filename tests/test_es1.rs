#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es1::anagrammi::sono_anagrammi;
    
    #[test]
    fn test_anagrammi() {
        assert_eq!(sono_anagrammi("roma", "amor"), true); // caso in cui le stringhe sono anagrammi quindi true
        assert_eq!(sono_anagrammi("roma",""), false); // caso in cui la secondaa stringa è vuota quindi false
        assert_eq!(sono_anagrammi("roma","insalata"), false); // caso in cui le stringhe non sono anagrammi e hanno lunghezza diversa quindi false
        assert_eq!(sono_anagrammi("","insalata"), false); // caso in cui la prima stringa è vuota quindi false
        assert_eq!(sono_anagrammi("roma","insa"), false); // caso in cui le stringhe non sono anagrammi e hanno stessa lunghezza quindi false
        assert_eq!(sono_anagrammi("calendario", "locandiera"),true);  // caso in cui le stringhe sono anagrammi quindi true
    }
}