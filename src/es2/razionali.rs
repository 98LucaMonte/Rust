use std::ops::{Add, Mul};

/** 
 * Questa è una struct che descrive i numeri Razionali ed è formata da due i32 che rappresentano
 * il numeratore e il denominatore.
*/

pub struct Razionali {
    numeratore: i32,
    denominatore: i32,
}

/** 
 * Questa è una implementazione del trait PartialEq utile a confrontare direttamente due razionali
*/

impl PartialEq for Razionali {
    fn eq(&self, other: &Self) -> bool {
        self.numeratore == other.numeratore && self.denominatore == other.denominatore
    }
}

/** 
 * Questa è una implementazione del trait Add utile a sommare direttamente due razionali.
 * Si calcola il mcm che sarà pari al prodotto dei denominatori diviso il loro mcd.
 * Il risultato verrà poi ridotto ai minimi termini.
*/

impl Add for Razionali {
    type Output = Razionali;

    fn add(self, rhs: Self) -> Self::Output {
        let mcm = self.denominatore * rhs.denominatore / Self::mcd(self.denominatore,rhs.denominatore);
        let mut raz = Self::new((self.numeratore*(mcm/self.denominatore))+(rhs.numeratore*(mcm/rhs.denominatore)), mcm);
        raz.riduzione_minimi_termini();
        raz
    }
}

/** 
 * Questa è una implementazione del trait Mul utile a moltiplicare direttamente due razionali.
 * Il risultato verrà poi ridotto ai minimi termini.
*/

impl Mul for Razionali {
    type Output = Razionali;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut raz = Self::new(self.numeratore*rhs.numeratore,self.denominatore*rhs.denominatore);
        raz.riduzione_minimi_termini();
        raz
    }
}

/** 
 * Questa è una implementazione del trait Add utile a sommare un intero con un razionale.
 * Il risultato verrà poi ridotto ai minimi termini.
*/

impl Add<i32> for Razionali {
    type Output = Razionali;

    fn add(self, rhs: i32) -> Self::Output {
        let raz = Self::int_to_raz(rhs);
        self + raz
    }
}

/** 
 * Questa è una implementazione del trait Mul utile a moltiplicare un intero con un razionale.
 * Il risultato verrà poi ridotto ai minimi termini.
*/

impl Mul<i32> for Razionali {
    type Output = Razionali;

    fn mul(self, rhs: i32) -> Self::Output {
        let raz = Self::int_to_raz(rhs);
        self * raz
    }
}

impl Razionali {
    
    /** 
     * Questa è un metodo utile per ridurre ai minimi termini un razionale. 
     * Tale metodo esegue il mcd tra numeratore e denominatore e modifica numeratore e denominatore
     * dividendoli per il mcd.
     * Se il mcd è minore di zero viene eseguito abs per trasformarlo in positivo
    */
    pub fn riduzione_minimi_termini(& mut self){
        let mcd = Self::mcd(self.numeratore, self.denominatore).abs();
        self.numeratore = self.numeratore/mcd;
        self.denominatore = self.denominatore/mcd;
    } 

    /** 
     * Questo è il costruttore che mi consente di allocare un nuovo razionale.
     * Panica se il denominatore è 0.
     * Se il numeratore è 0 allora il denominatore viene imposto a 1.
    */
    pub fn new(num:i32,den:i32) -> Razionali{
        if den == 0 {
            panic!("Il denominatore deve essere diverso da 0");
        }
        if num == 0 {
            return Razionali {numeratore:0,denominatore:1};
        }
        Razionali {numeratore:num,denominatore:den}
    }

    /** 
     * Questa è una funzione che converte un intero in un razionale impostando il denominatore a 1.
    */
    pub fn int_to_raz(num:i32) -> Razionali{
        Razionali {numeratore:num,denominatore:1}
    }

    /** 
     * Questa è una funzione utile a calcolare il mcd.
     * Il mcd viene calcolato usando il metodo di euclide.
    */
    fn mcd(mut a:i32,mut b:i32) -> i32{

        while b != 0{
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }

    /** 
     * Questa è una funzione che restituisce il numeratore del razionale
    */
    pub fn get_numeratore(&self)->i32{
        self.numeratore
    }

    /** 
     * Questa è una funzione che restituisce il denominatore del razionale
    */
    pub fn get_denominatore(&self)->i32{
        self.denominatore
    }
    
}
