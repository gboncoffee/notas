;; T representa a verdade, nil a falsidade.
;;
;; listp retorna t se o argumento é uma lista:
(listp '(a b c))
;; null retorna t se o argumento for uma lista vazia (ou nil, que é uma lista
;; vazia):
(null nil)
;; not é a função negadora (que na prática faz a mesma coisa que null):
(not nil)

;;
;; A condicional básica é o if, que também é um operador especial visto que só
;; evalua um dos parâmetros, o then ou o else:
(if (listp 42)
  (+ 4 2)  ;; then
  (+ 5 6)) ;; else
;; O else, claro, é opcional. Perceba que qualquer coisa fora nil evalua o then.

;; and irá evaluar todos os parâmetros até achar um falso, e então será
;; retornado nil. Caso todos sejam verdadeiros, o último será retornado. Perceba
;; que Lisp é preguiçosa e se encontrar um falso, nenhum argumento após esse
;; será evaluado.
;;
;; or é a mesma coisa, mas para assim que achar um verdadeiro.
