(define else #t)
(define (not x) (if x #f #t))
(define (> a b)
  (and
    (not (< a b))
    (not (= a b))))