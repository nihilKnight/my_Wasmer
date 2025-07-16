(module
  (func (export "test") (result i32)
    (local $i i32)
    (local $res i32)
    (local.set $i (i32.const 0))
    (loop $L
      ;; This part should be folded: (10 + 20) / 2 = 15
      i32.const 10
      i32.const 20
      i32.add
      i32.const 2
      i32.div_s
      local.set $res

      (local.set $i (i32.add (local.get $i) (i32.const 1)))
      (br_if $L (i32.lt_s (local.get $i) (i32.const 100)))
    )
    local.get $res
  )
) 