#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es7::slice_massimo_consecutivo::slice_massimo_consecutivo;

    #[test]
    fn test_slice() {
        // caso in cui lo slice massimo è quello centrale esempio preso dalla prof
        assert_eq!(slice_massimo_consecutivo(&[1,2,-4,0,1,3,-7,55,22], |x| *x >= 0), &[0,1,3]); 
        // caso in cui lo slice massimo è vuoto poiché nessun elemento dell'array soddisfa pred
        assert_eq!(slice_massimo_consecutivo(&[1,2,-4,0,1,3,-7,55,22], |x| *x > 100), &[]); 
        // caso in cui lo slice massimo è uguale a quello di partenza poiché tutti gli elementi dell'array soddisfano pred
        assert_eq!(slice_massimo_consecutivo(&[1,2,4,0,1,3,7,55,22], |x| *x >= 0), &[1,2,4,0,1,3,7,55,22]); 
        // caso in cui lo slice massimo è quello finale
        assert_eq!(slice_massimo_consecutivo(&[1,2,-4,0,1,3,-7,55,22,44,66,88], |x| *x % 2 == 0), &[22,44,66,88]); 
        // caso in cui lo slice massimo è quello iniziale
        assert_eq!(slice_massimo_consecutivo(&[33,66,99,1,2,-4,0,1,3,-7,55,22,44,66,88], |x| *x % 3 == 0), &[33,66,99]); 
    }
}