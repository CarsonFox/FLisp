(define else #t)

(define (not x)
  (if x
    #f
    #t))

(define (> a b)
  (not
    (or
      (< a b)
      (= a b))))