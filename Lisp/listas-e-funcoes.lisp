;; Listas (tudo):
;;
;; Escritas entre () e com itens de qualquer tipo (lembre-se de ' para não virar
;; uma chamada de função):
'(a lista (1 2 3) tem 3 "elementos")
;; A função list ajuda a criar listas. Como é uma função, seus argumentos são
;; evaluados a não ser que use ':
(list 'my (+ 2 1) "Sons")
;; Tudo são listas. Programas de Lisp são listas, e por isso programas de Lisp
;; podem gerar código de Lisp.
;;
;; Para representar listas vazias, usa-se () ou o símbolo nil
;;
;; A função cons adiciona elementos à frente de uma lista:
(cons 'a '(b c d))
;; A função car retorna o primeiro elemento da lista, e a função cdr retorna tudo
;; menos o primeiro elemento:
;;
;; Por uma combinação, seria possível por exemplo alcançar o terceiro elemento
;; da lista assim:
(car (cdr (cdr '(a b c d))))
;; Mas é possível fazer algo assim simplesmente chamando third:
(third '(a b c d))

;; 
;; Defina-se funções com defun:
(defun mythird (x) 
  (car (cdr (cdr x))))
