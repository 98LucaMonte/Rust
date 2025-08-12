
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
                    inizio_curr = 0;
                }
                len_curr = 0;

            }

            pos_att += 1;
            //println!("num:{}\tinizio:{}\tlen:{}\tinizio_curr:{}\tlen_curr:{}\tpos_att:{}",*x,inizio,len,inizio_curr,len_curr,pos_att);
            (inizio,len,inizio_curr,len_curr,pos_att)
        }  
    );
    
    if len_curr > len {
        inizio = inizio_curr;
        len = len_curr;
    }

    &array[inizio..inizio+len]
}

