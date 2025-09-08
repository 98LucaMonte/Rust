# Per tutti gli esercizi svolti finora
Prova a cambiare i metodi dei vari esercizi utilizzando il tipo Option per avere la possibilità di usare None.

Manca es8 da fare.
Implementare il protocollo English Auction. 

(BANDITORE) Un banditore attende n partecipanti ad un’asta per un prodotto che ha un prezzo iniziale ed 
un prezzo di  riserva al di sotto del quale non puo’ essere venduto.

Dopo aver ricevuto un messaggio da gli n partecipanti, invia loro la descrizione del prodotto che viene bandito e il suo prezzo minimo.
I partecipanti possono rispondere: dicendo che non vogliono partecipare all’asta oppure che 
offrono un valore che deve essere maggiore o uguale al prezzo minimo.

Il banditore se riceve da un partecipante il messaggio di uscita dall’asta lo elimina dalle successive 
interazioni, se invece riceve un messaggio con un prezzo, invia a tutti i partecipanti ancora 
interessati un nuovo prezzo (maggiore di quello che ha ricevuto da un partecipante).

Il protocollo va avanti così fino a che il banditore non ha ricevuto da tutti i partecipanti il messaggio 
che non vogliono partecipare.

A questo punto se il prezzo e’ maggiore del prezzo di riserva informa il partecipante che ha
ottenuto il prodotto che e’ il vincitore e gli altri che l’asta e’ finita.
Infine ill banditore mette in una struttura condivisa dai partecipanti l’indicazione del
prodotto venduto a quale cifra e’ stato venduto e a quale partecipante e’ stato assegnato.
I partecipanti leggono questa informazione. 

Partecipanti e banditore devono essere threads che comunicano usando canali e solo alla fine e’ 
Usata una struttura condivisa