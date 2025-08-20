/** 
 * Questa funzione prende in input un riferimento ad un array di interi ed un predicato "pred" che rappresenta una funzione
 * e ritorna lo slice dell'array che contiene il massimo numero di elementi consecutivi che verificano "pred".
 * Il predicato può essere ad esempio:
 * - elemento x dell'array > 0,
 * - elemento x dell'arrat % 2 == 0,
 * - elemento x dell'arrat % 2 != 0,
 * - ecc.
 * Usare il metodo fold di iterator per tale funzione.
*/

/* 
 * Per questa implementazione ho dichiarato 5 variabili ossia inizio, lunghezza, inizio_corrente, lunghezza_corrente e posizione attuale.
 * All'interno del fold è presenta una chiusura in cui all'interno avrò appunto queste 5 variabili più una variabile x che 
 * rappresenta l'elemento dell'array.
 * 
 * Se x soddisfa pred una funzione che prende in input un riferimento di un intero e restituisce un booleano 
 * allora se la lunghezza corrente è pari a 0 imposto inizio_corrente pari alla poszione attuale e incremento la lunghezza corrente.
 * 
 * Se invece x non soddisfa pred allora si verifica se la lunghezza corrente è maggiore della lunghezza e in tal caso
 * vuol dire che si è trovata una serie di elementi più lunga di quella precedente che può rappresentare lo slice da ritornare.
 * Infine, anche se len_curr non è maggiore di len si riporta len_curr = 0.
 * 
 * Per ogni iterazione di aumenta posizione attuale dell'array e si restituiscono le 5 variabili 
 * inizio, lunghezza, inizio_corrente, lunghezza_corrente e posizione_attuale.
 * 
 * Infine, con l'if finale controllo se len_curr è maggiore di len per verificare se l'ultima serie di elementi 
 * che soddisfava pred è effettivamente quella di lunghezza maggiore.
 * 
 * Restituisco poi il riferimento di array di i32 con uno slice che parte da inizio che rappresenta la posizione
 * da cui parte la serie di elementi che soddisfa pred fino a inizio+len esclusa. 
*/

pub fn slice_massimo_consecutivo<F>(array: &[i32], pred: F) -> &[i32]
where 
    F: Fn(&i32) -> bool
{
    let (mut inizio,mut len,inizio_curr,len_curr,_pos_att) = array.iter().fold(
        (0,0,0,0,0), 
        |(mut inizio,mut len, mut inizio_curr, mut len_curr, mut pos_att),x| {
            if pred(x){
                
                if len_curr == 0{
                    inizio_curr = pos_att;
                }

                len_curr += 1;
            
            }
            else{

                if len_curr > len{
                    len = len_curr;
                    inizio = inizio_curr;
                }
                len_curr = 0;

            }

            pos_att += 1;
            /*
             * println!("num:{}\tinizio:{}\tlen:{}\tinizio_curr:{}\tlen_curr:{}\tpos_att:{}",*x,inizio,len,inizio_curr,len_curr,pos_att);
             * Scommenta per vedere l'evoluzione della serie di elementi e verificare la correttezza del ragionamento
            */
            (inizio,len,inizio_curr,len_curr,pos_att)
        }  
    );
    
    if len_curr > len {
        inizio = inizio_curr;
        len = len_curr;
    }

    &array[inizio..inizio+len]
}

