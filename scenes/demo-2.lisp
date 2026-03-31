(include "./materials.lisp")

(set 's1
     (sphere
      (point 0 1 0) 1 blue))
(set 's2
     (sphere
      (point 2 0.5 2) 0.5 green))

(set 'mirror-dome
     (sphere
      (point 0 -17 0)
      30 dark-mirror))

(defun spiral-sphere (i n t)
  (sphere
   (progn
     (point
      (* 2 (cos (/ (* i 6.2) n)))
      (+ 0.5 (* 0.3 (cos (+ (/ (* i 6.2) n) (/ t 5.0)))))
      (* 2 (sin (/ (* i 6.2) n))))
     )
   0.2 red))

(defun spiral (scn i n t)
  (if (< i n)
      (scene-add
       (spiral scn (+ i 1) n t)
       (spiral-sphere i n t))
      scn))

(set 'p1
     (checkerboard
      (point 0 0 0)
      (vector 0 1 0)
      black white 0.5
      (vector 0.5 0 1)))

(set 'l1 (light (point 3 10 5) (color 1 1 1)))
(set 'l2 (light (point 2 10 5) (color 1 1 1)))


(set 'scn-base (scene
                (color 0.1 0.1 0.1)
                '(s1 s2 p1 mirror-dome)
                '(l1 l2)))

(set 'cam (camera (point 0 3 6) (point 0 0 0) (vector 0 1 0) 40 1920 1080))

(defun scene-fn (t)
  (spiral scn-base 0 30 t))

(defun cam-fn (t c)
  (let '((pos . (point -3 0.5 8))
         (cnt . (point 0 0 0))
         (to . (point -3 0.5 -8))
         (up . (vector 0 1 0))
         (fovy . 80)
         (pct . (/ t 300.0)))
    (let '((tpos . (+ pos (* (- to pos) pct)))
           (tfovy . (+ fovy (* 40 pct)))
           )
      (camera-reposition c tpos cnt up tfovy)
      )
    ))

(render-animation cam "demo-animation.mp4" scene-fn cam-fn 400 30 7 2)
