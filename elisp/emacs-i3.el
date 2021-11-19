(require 'windmove)

(defun my/emacs-i3-focus (dir)
  (let ((other-window (windmove-find-other-window dir)))
    (if (or (null other-window) (window-minibuffer-p other-window))
        nil ;; move focus out of emacs
      (progn (windmove-do-window-select dir) t))))

(defun my/emacs-i3-direction-exists-p (axis)
  (some (lambda (dir)
          (let ((win (windmove-find-other-window dir)))
            (and win (not (window-minibuffer-p win)))))
        (pcase axis
          ('width '(left right))
          ('height '(up down)))))

(defun my/emacs-i3-move (dir)
  (let ((other-window (windmove-find-other-window dir)))
    (if (and other-window (not (window-minibuffer-p other-window)))
        (progn (window-swap-states (selected-window) other-window) t)
      nil)))

(defun my/emacs-i3-resize (dir axis rest)
  ;; TODO take REST into account.
  (if (or (one-window-p)
          (not (my/emacs-i3-direction-exists-p axis)))
      nil ;; let i3 resize frame
    (pcase (list dir axis)
      ('(shrink width)
       (shrink-window-horizontally 1))
      ('(shrink height)
       (shrink-window-vertically 1))
      ('(grow width)
       (enlarge-window-horizontally 1))
      ('(grow height)
       (enlarge-window-vertically 1))
      (- nil))))

(defun my/emacs-i3-split (dir)
  "Split window into DIR and move focus to the new window"
  (if (pcase dir
        ('h (split-window-right))
        ('v (split-window-below)))
      (and (other-window 1) t)))

;; use-package transpose-frame
(defun my/emacs-i3-command (command)
  (pcase (split-string command)
    (`("focus" ,dir)
     (my/emacs-i3-focus (intern dir)))
    (`("move" ,dir)
     (my/emacs-i3-move (intern dir)))
    (`("resize" ,dir ,axis . ,rest)
     (my/emacs-i3-resize (intern dir) (intern axis) rest))
    (`("layout" "toggle" "split")
     (transpose-frame))
    (`("split" ,dir)
     (my/emacs-i3-split (intern dir)))
    (`("kill") (and (delete-window) t))
    (- nil)))

(provide 'emacs-i3)
