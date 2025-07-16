(module
  (memory (export "mem") 1)
  (func $init
    ;; store 42 at address calculated from constants
    ;; address is (4 * 8) + 12 = 44
    i32.const 4
    i32.const 8
    i32.mul
    i32.const 12
    i32.add
    i32.const 42 ;; value to store
    i32.store
  )

  (func (export "test") (result i32)
    call $init
    ;; read from the same address
    ;; address is (4 * 8) + 12 = 44
    i32.const 4
    i32.const 8
    i32.mul
    i32.const 12
    i32.add
    i32.load
  )
) 