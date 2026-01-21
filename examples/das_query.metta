;; On one node: Add merciful atom
(add-atom space (signed (!mercy-gate "eternal kindness propagates")))

;; On another: Query and auto-gate via wrapper
(query-remote "eternal kindness" (require-proof (!mercy-gate $result)))
