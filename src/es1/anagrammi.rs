
 
pub mod anagrammi { 
    use std::collections::HashMap;
    
    #[allow(unused)]
    pub fn sono_anagrammi(str1: &str, str2: &str) -> bool{

        if str1.is_empty() || str2.is_empty() || str1.len() != str2.len(){
            return false
        }

        let mut anagramma = HashMap::new();

        for s in str1.chars(){
            anagramma.entry(s).and_modify(|counter| *counter += 1).or_insert(1);
        }

        for s in str2.chars(){
            anagramma.entry(s).and_modify(|counter| *counter -= 1).or_insert(1);
        }

        for m in anagramma.values(){
            if *m != 0{
                return false
            }
        }
        true
    }
}    
