/** 
 * Questa funzione prende in input due letterali di stringa per cui hanno una dimensione immutabile
 * è restituisce un booleano che sarà true se le due stringhe sono anagrammi false altrimenti.
*/

use std::collections::HashMap;

pub fn sono_anagrammi(str1: &str, str2: &str) -> bool{

    if str1.is_empty() || str2.is_empty() || str1.len() != str2.len(){
        return false
    }

    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();

    for s in str1.chars(){
        map1.entry(s).and_modify(|counter| *counter += 1).or_insert(1);
    }

    for s in str2.chars(){
        map2.entry(s).and_modify(|counter| *counter += 1).or_insert(1);
    }

    map1 == map2
}
