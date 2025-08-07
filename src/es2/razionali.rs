use std::ops::{Add, Mul};

pub struct Razionali {
    pub numeratore: i32,
    pub denominatore: i32,
}

impl PartialEq for Razionali {
    fn eq(&self, other: &Self) -> bool {
        self.numeratore == other.numeratore && self.denominatore == other.denominatore
    }
}

impl Add for Razionali {
    type Output = Razionali;

    fn add(self, rhs: Self) -> Self::Output {
        let mcm = self.denominatore * rhs.denominatore / Self::mcd(self.denominatore,rhs.denominatore);
        let mut raz = Self::new((self.numeratore*(mcm/self.denominatore))+(rhs.numeratore*(mcm/rhs.denominatore)), mcm);
        raz.riduzione_minimi_termini();
        raz
    }
}

impl Mul for Razionali {
    type Output = Razionali;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut raz = Self::new(self.numeratore*rhs.numeratore,self.denominatore*rhs.denominatore);
        raz.riduzione_minimi_termini();
        raz
    }
}

impl Add<i32> for Razionali {
    type Output = Razionali;

    fn add(self, rhs: i32) -> Self::Output {
        let raz = Self::int_to_raz(rhs);
        self + raz
    }
}

impl Mul<i32> for Razionali {
    type Output = Razionali;

    fn mul(self, rhs: i32) -> Self::Output {
        let raz = Self::int_to_raz(rhs);
        self * raz
    }
}

impl Razionali {
    
    pub fn riduzione_minimi_termini(& mut self){
        let mut mcd = Self::mcd(self.numeratore, self.denominatore);
        if mcd < 0{
            mcd = mcd * -1;
        }
        self.numeratore = self.numeratore/mcd;
        self.denominatore = self.denominatore/mcd;
    } 

    pub fn new(num:i32,den:i32) -> Razionali{
        if den == 0 {
            panic!("Il denominatore deve essere diverso da 0");
        }
        if num == 0 {
            return Razionali {numeratore:0,denominatore:1};
        }
        Razionali {numeratore:num,denominatore:den}
    }

    pub fn int_to_raz(num:i32) -> Razionali{
        Razionali {numeratore:num,denominatore:1}
    }

    fn mcd(mut a:i32,mut b:i32) -> i32{

        while b != 0{
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
    
}
