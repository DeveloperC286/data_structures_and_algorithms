
(defun sum_list (n) (cond ((null n) 0) (t (+ (car n) (sum_list (cdr n))))))
