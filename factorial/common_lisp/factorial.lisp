
(defun factorial (n)
  (cond ((< n 1) nil) ((= n 1) 1) ((= n 2) 2) (t (* n (factorial (decf n 1))))))
