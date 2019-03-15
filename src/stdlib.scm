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

(define (odd? x)
  (=
    (remainder x 2)
    1))

(define (even? x)
  (not (odd? x)))

(define (inc x)
  (+ x 1))