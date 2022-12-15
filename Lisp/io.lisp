;; A função mais básica de IO é print, que mostra Lisp em si (ou seja, strings
;; tem aspas, etc):
(print "Hello, World!")

;; a função format formata:
(format 
  t                            ;; indica output padrão
  "~A mais ~A é igual a ~A.~%" ;; template, ~A é parâmetro e ~% é newline
  2                            ;; parâmetro
  3                            ;; parâmetro
  (+ 2 3))                     ;; parâmetro (será evaluado, claro)

;; Para receber input, função read. Para criar uma função semelhante à input()
;; de Python, assim:
(defun input (string)
  (format t string)
  (read))

;; Nota que read é um parser de Lisp completo.
