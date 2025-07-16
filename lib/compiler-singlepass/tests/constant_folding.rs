//! Tests for constant folding in singlepass backend

use wasmer::{imports, Instance, Module, Store, Value, wat2wasm};
use wasmer_compiler_singlepass::Singlepass;
use std::fs;

fn run_wasm_and_get_export(module_bytes: &[u8], export: &str) -> i32 {
    let compiler = Singlepass::default();
    let mut store = Store::new(compiler);
    let module = Module::new(&store, module_bytes).expect("module compile");
    let import_object = imports! {};
    let instance = Instance::new(&mut store, &module, &import_object).expect("instance");
    let func = instance.exports.get_function(export).expect("get export");
    let result = func.call(&mut store, &[]).expect("call");
    match result[0] {
        Value::I32(val) => val,
        _ => panic!("unexpected return type"),
    }
}
#[test]
fn test_simple_add() {
    let binding = fs::read("./tests/const_folding/simple_add.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let result = run_wasm_and_get_export(&wasm, "add");
    assert_eq!(result, 42);
}

#[test]
fn test_simple_sub() {
    let binding = fs::read("./tests/const_folding/simple_sub.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let result = run_wasm_and_get_export(&wasm, "sub");
    assert_eq!(result, 42);
}

#[test]
fn test_simple_mul() {
    let binding = fs::read("./tests/const_folding/simple_mul.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let result = run_wasm_and_get_export(&wasm, "mul");
    assert_eq!(result, 42);
}

#[test]
fn test_simple_chain() {
    let binding = fs::read("./tests/const_folding/simple_chain.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let result = run_wasm_and_get_export(&wasm, "chain");
    // (2 + 3) * 4 - 5 = 15
    assert_eq!(result, 15);
}

#[test]
fn test_simple_cmp() {
    let binding = fs::read("./tests/const_folding/simple_cmp.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let eq = run_wasm_and_get_export(&wasm, "cmp_eq");
    let lt = run_wasm_and_get_export(&wasm, "cmp_lt");
    assert_eq!(eq, 1);
    assert_eq!(lt, 1);
}

#[test]
fn test_matrix_init() {
    let binding = fs::read("./tests/const_folding/matrix_init.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let result = run_wasm_and_get_export(&wasm, "test");
    assert_eq!(result, 42);
}

#[test]
fn test_loop_invariant() {
    let binding = fs::read("./tests/const_folding/loop_invariant.wast").unwrap();
    let wasm = wat2wasm(&binding).unwrap();
    let result = run_wasm_and_get_export(&wasm, "test");
    assert_eq!(result, 15);
}