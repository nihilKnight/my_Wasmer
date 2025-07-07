(module
  (func (export "cmp_eq") (result i32)
    i32.const 42
    i32.const 42
    i32.eq
  )
  (func (export "cmp_lt") (result i32)
    i32.const 1
    i32.const 2
    i32.lt_s
  )
) 