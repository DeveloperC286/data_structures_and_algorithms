
(defun largest_number (n)
  (cond ((null n) nil) ((= (length n) 1) (car n))
        ((>= (car n) (largest_number (cdr n))) (car n))
        ((> (largest_number (cdr n)) (car n)) (largest_number (cdr n)))))
