#[cfg(test)]
mod tests {  
    use esercizi_esame_rust::es6::list::List;
    
    /**
     * Test utile per verificare se l'inserimento di nuovi nodi nella lista.
     * Si verifica anche se la lista mantiene l'ordine degli elementi come previsto.
    */
    #[test]
    fn test_aggiungi_nodo_in_lista() {
        
        let mut lista = List::new();
        assert_eq!(lista.is_empty(),true);
        
        lista.add_el(0, 42);
        assert_eq!(lista.list_len(),1);
        assert_eq!(lista.is_empty(),false);
        assert_eq!(lista.get_el(0),Some(&42));
        assert_eq!(lista.get_el(1),None);
        assert_eq!(lista.get_el(44),None);

        lista.add_el(1, 22);
        assert_eq!(lista.list_len(),2);
        assert_eq!(lista.get_el(0),Some(&42));
        assert_eq!(lista.get_el(1),Some(&22));
        assert_eq!(lista.get_el(2),None);
        assert_eq!(lista.get_el(44),None);

        
        lista.add_el(1, 33);
        assert_eq!(lista.list_len(),3);
        assert_eq!(lista.get_el(0),Some(&42));
        assert_eq!(lista.get_el(1),Some(&33));
        assert_eq!(lista.get_el(2),Some(&22));
        assert_eq!(lista.get_el(3),None);
        assert_eq!(lista.get_el(111),None);

        
        lista.add_el(3, 55);
        assert_eq!(lista.list_len(),4);
        assert_eq!(lista.get_el(0),Some(&42));
        assert_eq!(lista.get_el(1),Some(&33));
        assert_eq!(lista.get_el(2),Some(&22));
        assert_eq!(lista.get_el(3),Some(&55));
        assert_eq!(lista.get_el(4),None);
        assert_eq!(lista.get_el(4224),None);

        
        lista.add_el(66, 66);
        assert_eq!(lista.list_len(),5);
        assert_eq!(lista.get_el(0),Some(&42));
        assert_eq!(lista.get_el(1),Some(&33));
        assert_eq!(lista.get_el(2),Some(&22));
        assert_eq!(lista.get_el(3),Some(&55));
        assert_eq!(lista.get_el(4),Some(&66));
        assert_eq!(lista.get_el(5),None);
        assert_eq!(lista.get_el(33),None);

    }
    
    /**
     * Test utile per verificare se la rimozione dei nodi nella lista.
     * Si verifica anche se la lista mantiene l'ordine degli elementi come previsto.
    */
    #[test]
    fn test_rimuovi_nodo_in_lista() {

        let mut lista = List::new();
        lista.add_el(0, 42);
        lista.add_el(1, 22);
        lista.add_el(1, 33);
        lista.add_el(3, 55);
        lista.add_el(66, 66);
        assert_eq!(lista.list_len(),5);
    
        lista.rm_el(0);
        assert_eq!(lista.list_len(),4);
        assert_eq!(lista.get_el(0),Some(&33));
        assert_eq!(lista.get_el(1),Some(&22));
        assert_eq!(lista.get_el(2),Some(&55));
        assert_eq!(lista.get_el(3),Some(&66));
        assert_eq!(lista.get_el(4),None);
        assert_eq!(lista.get_el(33),None);

        lista.rm_el(3);
        assert_eq!(lista.list_len(),3);
        assert_eq!(lista.get_el(0),Some(&33));
        assert_eq!(lista.get_el(1),Some(&22));
        assert_eq!(lista.get_el(2),Some(&55));
        assert_eq!(lista.get_el(3),None);
        assert_eq!(lista.get_el(44),None);

        lista.rm_el(1);
        assert_eq!(lista.list_len(),2);
        assert_eq!(lista.get_el(0),Some(&33));
        assert_eq!(lista.get_el(1),Some(&55));
        assert_eq!(lista.get_el(2),None);
        assert_eq!(lista.get_el(22),None);
    }
}