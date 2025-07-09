# Constant Folding Bench Tests

这个bench测试套件用于评估singlepass编译器中的constant folding优化效果。

## 测试场景

### 1. Constant Folding Enabled vs Disabled
比较开启和关闭constant folding时的性能差异：

- `constant_folding_enabled`: 包含大量constant folding机会的函数
- `constant_folding_disabled`: 同样的函数但关闭constant folding
- `dynamic_operations_enabled/disabled`: 使用动态值的函数（无constant folding机会）
- `loop_with_constants_enabled/disabled`: 循环中包含constant folding的函数

### 2. Constant Folding Depth Impact
测试不同constant folding深度限制对性能的影响：
- depth_1: 最大深度1
- depth_5: 最大深度5  
- depth_10: 最大深度10
- depth_20: 最大深度20

### 3. Loop Iterations Impact
测试不同循环迭代次数对constant folding效果的影响：
- 10_iterations: 10次迭代
- 100_iterations: 100次迭代
- 1000_iterations: 1000次迭代
- 10000_iterations: 10000次迭代

## 运行测试

```bash
# 运行所有bench测试
cargo bench

# 运行特定测试组
cargo bench constant_folding_enabled_vs_disabled
cargo bench constant_folding_depth_impact
cargo bench loop_iterations_impact

# 生成HTML报告
cargo bench -- --output-format html
```

## 预期结果

1. **Constant Folding Enabled vs Disabled**: 开启constant folding的函数应该比关闭时运行更快，因为编译时计算了常量表达式。

2. **Depth Impact**: 较高的深度限制应该允许更多的constant folding，但可能增加编译时间。

3. **Loop Impact**: 在循环中，constant folding的效果应该随着迭代次数增加而更加明显。

## 测试函数说明

### constant_folding_heavy
包含多种constant folding场景的函数：
- 算术运算链：`42 + 17 * 8 - 100`
- 位运算：`(0xFF00FF00 & 0x00FF00FF) << 16 | 0x0000FFFF`
- 比较运算：`(100 > 50) && (200 < 150) == 1`
- 数学表达式：`((2 * 3 + 4) / 2) - 1`
- 位计数和旋转：`popcnt(0xAAAAAAAA) == 16` 和 `rotl(0x12345678, 8) == 0x56781234`

### dynamic_operations
执行相同操作但使用动态参数，没有constant folding机会。

### loop_with_constants
循环函数，每次迭代都包含可折叠的常量表达式。

## 配置选项

在`config.rs`中可以调整以下参数：
- `enable_constant_folding`: 是否启用constant folding
- `max_constant_folding_depth`: constant folding的最大深度限制 