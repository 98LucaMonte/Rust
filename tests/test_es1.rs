#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es1::anagrammi::sono_anagrammi;
    
    #[test]
    fn test_anagrammi() {
        assert_eq!(sono_anagrammi("roma", "amor"), true);
        assert_eq!(sono_anagrammi("roma",""), false);
        assert_eq!(sono_anagrammi("roma","insalata"), false);
        assert_eq!(sono_anagrammi("","insalata"), false);
        assert_eq!(sono_anagrammi("roma","insa"), false);
        assert_eq!(sono_anagrammi("calendario", "locandiera"),true);  
    }
}