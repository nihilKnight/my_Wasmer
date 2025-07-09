use criterion::{black_box, criterion_group, criterion_main, Criterion};
use wasmer::{imports, sys::EngineBuilder, wat2wasm, Instance, Module, Store, Value};
use wasmer_compiler_singlepass::Singlepass;

/// Generate a WAT module with constant folding opportunities
fn generate_constant_folding_module() -> String {
    r#"
    (module
        ;; Function with many constant folding opportunities
        (func $constant_folding_heavy (param i32) (result i32)
            ;; Arithmetic chain with constants
            i32.const 42
            i32.const 17
            i32.add
            i32.const 8
            i32.mul
            i32.const 100
            i32.sub
            
            ;; Bit operations with constants
            i32.const 0xFF00FF00
            i32.const 0x00FF00FF
            i32.and
            i32.const 16
            i32.shl
            i32.const 0x0000FFFF
            i32.or
            
            ;; Comparison operations
            i32.const 100
            i32.const 50
            i32.gt_s
            i32.const 200
            i32.const 150
            i32.lt_s
            i32.and
            i32.const 1
            i32.eq
            
            ;; Mathematical expressions
            i32.const 2
            i32.const 3
            i32.mul
            i32.const 4
            i32.add
            i32.const 2
            i32.div_s
            i32.const 1
            i32.sub
            
            ;; Bit counting and rotation
            i32.const 0xAAAAAAAA
            i32.popcnt
            i32.const 16
            i32.eq
            
            i32.const 0x12345678
            i32.const 8
            i32.rotl
            i32.const 0x56781234
            i32.eq
            
            ;; Combine all results
            i32.add
            i32.add
            i32.add
            i32.add
            i32.add
            
            ;; Use parameter for final computation
            local.get 0
            i32.add
        )
        
        ;; Function with the same operations but using dynamic values
        (func $dynamic_operations (param i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32 i32) (result i32)
            ;; Same operations but using parameters
            local.get 0
            local.get 1
            i32.add
            local.get 2
            i32.mul
            local.get 3
            i32.sub
            
            local.get 4
            local.get 5
            i32.and
            local.get 6
            i32.shl
            local.get 7
            i32.or
            
            local.get 8
            local.get 9
            i32.gt_s
            local.get 10
            local.get 11
            i32.lt_s
            i32.and
            local.get 12
            i32.eq
            
            local.get 13
            local.get 14
            i32.mul
            local.get 15
            i32.add
            local.get 16
            i32.div_s
            local.get 17
            i32.sub
            
            local.get 18
            i32.popcnt
            local.get 19
            i32.eq
            
            ;; Combine all results
            i32.add
            i32.add
            i32.add
            i32.add
        )
        
        ;; Function with loop containing constant folding
        (func $loop_with_constants (param i32) (result i32)
            (local $sum i32)
            (local $i i32)
            
            i32.const 0
            local.set $sum
            i32.const 0
            local.set $i
            
            (block $loop_exit
                (loop $main_loop
                    local.get $i
                    local.get 0
                    i32.ge_s
                    br_if $loop_exit ;; Correct: exit loop when i >= param
                    
                    ;; Add constant expressions that can be folded
                    local.get $sum
                    i32.const 42
                    i32.const 17
                    i32.add
                    i32.const 8
                    i32.mul
                    i32.const 100
                    i32.sub
                    i32.add
                    local.set $sum
                    
                    local.get $i
                    i32.const 1
                    i32.add
                    local.set $i
                    
                    br $main_loop ;; Continue loop
                )
            )
            
            local.get $sum
        )
        
        (export "constant_folding_heavy" (func $constant_folding_heavy))
        (export "dynamic_operations" (func $dynamic_operations))
        (export "loop_with_constants" (func $loop_with_constants))
    )
    "#
    .to_string()
}

fn bench_constant_folding_enabled_vs_disabled(c: &mut Criterion) {
    let mut group = c.benchmark_group("constant_folding_enabled_vs_disabled");
    
    // Test with constant folding enabled
    {
        let config = Singlepass::new();
        let engine = EngineBuilder::new(config);
        let mut store = Store::new(engine);
        
        let wat = generate_constant_folding_module();
        let wasm_bytes = wat2wasm(wat.as_bytes()).unwrap();
        let module = Module::new(&store, wasm_bytes).unwrap();
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        
        let constant_func = instance.exports.get_function("constant_folding_heavy").unwrap();
        let dynamic_func = instance.exports.get_function("dynamic_operations").unwrap();
        let loop_func = instance.exports.get_function("loop_with_constants").unwrap();
        
        group.bench_function("constant_folding_enabled", |b| {
            b.iter(|| {
                let result = constant_func.call(&mut store, &[Value::I32(black_box(42))]).unwrap();
                result[0].unwrap_i32()
            });
        });
        
        group.bench_function("dynamic_operations_enabled", |b| {
            b.iter(|| {
                let params = vec![
                    Value::I32(42), Value::I32(17), Value::I32(8), Value::I32(100),
                    Value::I32(0xFF00FF00u32 as i32), Value::I32(0x00FF00FF), Value::I32(16), Value::I32(0x0000FFFF),
                    Value::I32(100), Value::I32(50), Value::I32(200), Value::I32(150),
                    Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4),
                    Value::I32(2), Value::I32(1), Value::I32(0xAAAAAAAAu32 as i32), Value::I32(16)
                ];
                let result = dynamic_func.call(&mut store, &params).unwrap();
                result[0].unwrap_i32()
            });
        });
        
        group.bench_function("loop_with_constants_enabled", |b| {
            b.iter(|| {
                let result = loop_func.call(&mut store, &[Value::I32(black_box(100))]).unwrap();
                result[0].unwrap_i32()
            });
        });
    }
    
    // Test with constant folding disabled
    {
        let mut config = Singlepass::new();
        config.enable_constant_folding = false;
        let engine = EngineBuilder::new(config);
        let mut store = Store::new(engine);
        
        let wat = generate_constant_folding_module();
        let wasm_bytes = wat2wasm(wat.as_bytes()).unwrap();
        let module = Module::new(&store, wasm_bytes).unwrap();
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        
        let constant_func = instance.exports.get_function("constant_folding_heavy").unwrap();
        let dynamic_func = instance.exports.get_function("dynamic_operations").unwrap();
        let loop_func = instance.exports.get_function("loop_with_constants").unwrap();
        
        group.bench_function("constant_folding_disabled", |b| {
            b.iter(|| {
                let result = constant_func.call(&mut store, &[Value::I32(black_box(42))]).unwrap();
                result[0].unwrap_i32()
            });
        });
        
        group.bench_function("dynamic_operations_disabled", |b| {
            b.iter(|| {
                let params = vec![
                    Value::I32(42), Value::I32(17), Value::I32(8), Value::I32(100),
                    Value::I32(0xFF00FF00u32 as i32), Value::I32(0x00FF00FF), Value::I32(16), Value::I32(0x0000FFFF),
                    Value::I32(100), Value::I32(50), Value::I32(200), Value::I32(150),
                    Value::I32(1), Value::I32(2), Value::I32(3), Value::I32(4),
                    Value::I32(2), Value::I32(1), Value::I32(0xAAAAAAAAu32 as i32), Value::I32(16)
                ];
                let result = dynamic_func.call(&mut store, &params).unwrap();
                result[0].unwrap_i32()
            });
        });
        
        group.bench_function("loop_with_constants_disabled", |b| {
            b.iter(|| {
                let result = loop_func.call(&mut store, &[Value::I32(black_box(100))]).unwrap();
                result[0].unwrap_i32()
            });
        });
    }
    
    group.finish();
}

fn bench_constant_folding_depth_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("constant_folding_depth_impact");
    
    // Test different constant folding depths
    for depth in [1, 5, 10, 20] {
        let mut config = Singlepass::new();
        config.max_constant_folding_depth = depth;
        let engine = EngineBuilder::new(config);
        let mut store = Store::new(engine);
        
        let wat = generate_constant_folding_module();
        let wasm_bytes = wat2wasm(wat.as_bytes()).unwrap();
        let module = Module::new(&store, wasm_bytes).unwrap();
        let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
        
        let constant_func = instance.exports.get_function("constant_folding_heavy").unwrap();
        
        group.bench_function(&format!("depth_{}", depth), |b| {
            b.iter(|| {
                let result = constant_func.call(&mut store, &[Value::I32(black_box(42))]).unwrap();
                result[0].unwrap_i32()
            });
        });
    }
    
    group.finish();
}

fn bench_loop_iterations_impact(c: &mut Criterion) {
    let mut group = c.benchmark_group("loop_iterations_impact");
    
    let config = Singlepass::new();
    let engine = EngineBuilder::new(config);
    let mut store = Store::new(engine);
    
    let wat = generate_constant_folding_module();
    let wasm_bytes = wat2wasm(wat.as_bytes()).unwrap();
    let module = Module::new(&store, wasm_bytes).unwrap();
    let instance = Instance::new(&mut store, &module, &imports! {}).unwrap();
    
    let loop_func = instance.exports.get_function("loop_with_constants").unwrap();
    
    // Test different loop sizes
    for iterations in [10, 100, 1000, 10000] {
        group.bench_function(&format!("{}_iterations", iterations), |b| {
            b.iter(|| {
                let result = loop_func.call(&mut store, &[Value::I32(black_box(iterations))]).unwrap();
                result[0].unwrap_i32()
            });
        });
    }
    
    group.finish();
}

criterion_group!(
    benches,
    bench_constant_folding_enabled_vs_disabled,
    bench_constant_folding_depth_impact,
    bench_loop_iterations_impact
);
criterion_main!(benches); 