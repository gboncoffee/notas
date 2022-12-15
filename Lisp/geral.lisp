;; Common Lisp tem literalmente só duas sintaxes:

1       ;; expressão atômica
(+ 1 2) ;; expressão comum

;; Isso ocorre porque código de Lisp é só um monte de listas. Literalmente uma
;; estrutura de dados.
;;
;; Tudo é uma lista, funções são listas.

;; Perceba que a linguagem usa reverse-polish notation e permite mais de um
;; operando ser passado pra certos operadores que em sintaxe normal não seria
;; possível:
(/ 1 2 3)

;; Já coloco um operador bem útil: ' (ou quote), que impede expressões de serem
;; calculadas:
'(/ 1 2 3)        ;; retorna 
(quote (/ 1 2 3)) ;; literalmente "(/ 1 2 3)"

;; Tipos:
42              ;; integer
"Hello, World!" ;; string
'MyName         ;; symbol, case-insensitive
(42 69)         ;; lista (literalmente tudo)
