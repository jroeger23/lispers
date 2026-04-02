(set 'red
     (material
      (color 1 0 0)
      (color 1 0 0)
      (color 0.5 0 0)
      50 0.25))
(set 'blue
     (material
      (color 0 0 1)
      (color 0 0 1)
      (color 0 0 0.6)
      50 0.25))
(set 'green
     (material
      (color 0 1 0)
      (color 0 1 0)
      (color 0 0.6 0)
      50 0.25))

(set 's1
     (sphere
      (point 0 1 0) 1 blue))
(set 's2
     (sphere
      (point 2 0.5 2) 0.5 green))

(defun spiral-sphere (i n)
  (sphere
   (progn
     (print "Spiral Sphere at: ")
     (println (point
               (* 2 (cos (/ (* i 6.2) n)))
               0.5
               (* 2 (sin (/ (* i 6.2) n)))))
     )
   0.5 red))

(defun spiral (scn i n)
  (if (< i n)
      (scene-add
       (spiral scn (+ i 1) n)
       (spiral-sphere i n))
      scn))

(set 'mandelbrot
     (mandelbrot-texture
      1800.0
      (point2 -0.7489967346191402 -0.06952285766601607)
      1000
      (color 0.3 0 0)
      (color 0.3 0 0)
      (color 0.3 0 0)
      ))

(set 'p1
     (texture-plane
      mandelbrot
      (point 0 0 0)
      (vector 0 1 0)
      1.0
      (vector 1 0 0)))

(set 'l1 (light (point 3 10 5) (color 1 1 1)))
(set 'l2 (light (point 2 10 5) (color 1 1 1)))


(set 'scn (scene
           (color 0.1 0.1 0.1)
           '(s1 s2 p1)
           '(l1 l2)))

(println (cons "Final Scene:" scn))

(set 'cam (camera (point 0 3 6) (point 0 0 0) (vector 0 1 0) 40 1920 1080))

(render cam scn 5 4 "demo-3.png")
