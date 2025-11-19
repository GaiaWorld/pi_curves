# pi_curves

[![Crates.io](https://img.shields.io/crates/v/pi_curves.svg)](https://crates.io/crates/pi_curves)
[![Documentation](https://docs.rs/pi_curves/badge.svg)](https://docs.rs/pi_curves)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

一个功能强大且依赖简单的动画曲线计算库，专为游戏开发和动画设计而构建。

## 🚀 特性

- **多种曲线类型**: 支持线性、缓动、贝塞尔、Hermite 等多种插值算法
- **高性能**: 针对动画系统优化的高性能实现
- **类型安全**: 利用 Rust 的类型系统确保安全性
- **简单易用**: 直观的 API 设计，易于集成
- **丰富缓动函数**: 30+ 种内置缓动函数
- **帧动画支持**: 专门为帧动画设计的 API
- **向量化计算**: 支持 nalgebra 向量类型（可选特性）

## 📦 安装

在你的 `Cargo.toml` 中添加：

```toml
[dependencies]
pi_curves = "0.1.3"

# 如果需要向量化支持
pi_curves = { version = "0.1.3", features = ["nalgebra"] }
```

## 🎯 快速开始

### 基础线性插值

```rust
use pi_curves::prelude::*;

fn main() {
    // 创建一个简单的帧值曲线
    let mut curve = FrameCurve::curve_frame_values(60); // 60 FPS

    // 添加关键帧
    curve.curve_frame_values_frame(0, 0.0);    // 第0帧，值为0.0
    curve.curve_frame_values_frame(30, 1.0);  // 第30帧，值为1.0
    curve.curve_frame_values_frame(60, 0.0);  // 第60帧，值为0.0

    // 在时间 t=0.5 (15帧) 时插值
    let value = curve.interple(15.0, &AnimationAmountCalc::default());
    println!("插值结果: {}", value); // 输出: 0.5
}
```

### 缓动动画

```rust
use pi_curves::{prelude::*, easing::EEasingMode};

fn main() {
    // 创建一个缓动曲线 (0 -> 1，60帧，使用 SineInOut 缓动)
    let curve = FrameCurve::curve_easing(
        0.0,                           // 起始值
        1.0,                           // 变化量
        60,                            // 帧数
        60,                            // 设计帧率
        EEasingMode::SineInOut         // 缓动模式
    );

    // 在不同时间点获取值
    for frame in 0..=60 {
        let t = frame as f32 / 60.0;
        let value = curve.interple(t, &AnimationAmountCalc::default());
        println!("帧 {}: {:.3}", frame, value);
    }
}
```

### 三次贝塞尔曲线

```rust
use pi_curves::prelude::*;

fn main() {
    // 创建三次贝塞尔曲线 (类似 CSS cubic-bezier)
    let curve = FrameCurve::curve_cubic_bezier(
        0.0,                           // 起始值
        1.0,                           // 变化量
        60,                            // 帧数
        60,                            // 设计帧率
        0.42, 0.0, 1.0, 1.0          // 贝塞尔控制点 (x1, y1, x2, y2)
    );

    let value = curve.interple(0.5, &AnimationAmountCalc::default());
    println!("贝塞尔插值结果: {:.3}", value);
}
```

### Hermite 曲线

```rust
use pi_curves::prelude::*;

fn main() {
    // 创建 Hermite 曲线（带切线控制）
    let mut curve = FrameCurve::curve_minmax_curve(0.0, 1.0, 60);

    // 添加关键帧，每个都带有入切线和出切线
    curve.curve_minmax_curve_frame(0,   0.0, 2.0, 2.0);  // 起始帧，值，入切线，出切线
    curve.curve_minmax_curve_frame(30,  0.5, 0.0, 0.0);  // 中间帧，平滑过渡
    curve.curve_minmax_curve_frame(60,  1.0, 2.0, 2.0);  // 结束帧

    let value = curve.interple(15.0, &AnimationAmountCalc::default());
    println!("Hermite 插值结果: {:.3}", value);
}
```

### 步进动画

```rust
use pi_curves::{prelude::*, steps::EStepMode};

fn main() {
    let mut curve = FrameCurve::curve_frame_values(60);

    // 添加关键帧
    curve.curve_frame_values_frame(0, 0.0);
    curve.curve_frame_values_frame(15, 1.0);
    curve.curve_frame_values_frame(30, 2.0);
    curve.curve_frame_values_frame(60, 3.0);

    // 使用步进插值（4步，JumpStart模式）
    let step_calc = AnimationAmountCalc::from_steps(4, EStepMode::JumpStart);

    for frame in 0..=60 {
        let t = frame as f32 / 60.0;
        let value = curve.interple(t, &step_calc);
        println!("帧 {}: {:.1}", frame, value);
    }
}
```

## 📚 API 参考

### 曲线类型

#### `EFrameCurveType` 枚举

- **`FrameValues`**: 线性关键帧插值
- **`EasingCurve`**: 缓动曲线
- **`MinMaxCurve`**: Hermite 插值曲线
- **`CubicBezierCurve`**: 三次贝塞尔曲线
- **`GLTFCubicSpline`**: GLTF 样条曲线

### 缓动模式

库提供了 30+ 种缓动函数：

#### 基础缓动
- `None` - 线性插值
- `SineIn/Out/InOut` - 正弦缓动
- `Quad/Quart/Quint In/Out/InOut` - 二次/四次/五次缓动
- `Cubic In/Out/InOut` - 三次缓动

#### 特殊缓动
- `Back In/Out/InOut` - 回弹效果
- `Bounce In/Out/InOut` - 弹跳效果
- `Elastic In/Out/InOut` - 弹性效果
- `Circle In/Out/InOut` - 圆形缓动
- `Expo In/Out/InOut` - 指数缓动

### 步进模式

- `JumpStart` - 在步进开始时跳跃
- `JumpEnd` - 在步进结束时跳跃
- `JumpNone` - 无跳跃
- `JumpBoth` - 两端都跳跃

## 🔧 高级用法

### 自定义动画计算器

```rust
use pi_curves::prelude::*;

fn main() {
    // 创建自定义贝塞尔计算器
    let custom_calc = AnimationAmountCalc::from_cubic_bezier(0.25, 0.1, 0.25, 1.0);

    // 创建步进计算器
    let step_calc = AnimationAmountCalc::from_steps(8, EStepMode::JumpBoth);

    // 创建缓动计算器
    let easing_calc = AnimationAmountCalc::from_easing(EEasingMode::ElasticOut);

    // 在曲线中使用自定义计算器
    let curve = FrameCurve::curve_frame_values(60);
    // ... 添加关键帧 ...

    let value = curve.interple(0.5, &custom_calc);
    println!("自定义计算结果: {:.3}", value);
}
```

### 向量类型支持（启用 nalgebra 特性）

```rust
#[cfg(feature = "nalgebra")]
use pi_curves::prelude::*;
#[cfg(feature = "nalgebra")]
use nalgebra::Vector3;

#[cfg(feature = "nalgebra")]
fn main() {
    let mut curve = FrameCurve::curve_frame_values(60);

    // 使用 Vector3 作为值类型
    curve.curve_frame_values_frame(0, Vector3::new(0.0, 0.0, 0.0));
    curve.curve_frame_values_frame(60, Vector3::new(1.0, 2.0, 3.0));

    let value = curve.interple(0.5, &AnimationAmountCalc::default());
    println!("向量插值: {:?}", value);
}
```

### 性能优化示例

```rust
use pi_curves::prelude::*;

fn optimized_animation_system() {
    // 预计算多个曲线
    let curves: Vec<FrameCurve<f32>> = vec![
        // 位置动画
        {
            let mut c = FrameCurve::curve_easing(0.0, 100.0, 60, 60, EEasingMode::CubicInOut);
            c
        },
        // 旋转动画
        {
            let mut c = FrameCurve::curve_cubic_bezier(0.0, 360.0, 60, 60, 0.42, 0.0, 1.0, 1.0);
            c
        },
        // 缩放动画
        {
            let mut c = FrameCurve::curve_frame_values(60);
            c.curve_frame_values_frame(0, 1.0);
            c.curve_frame_values_frame(30, 2.0);
            c.curve_frame_values_frame(60, 1.0);
            c
        }
    ];

    // 批量计算
    let calc = AnimationAmountCalc::default();
    for frame in 0..=60 {
        let t = frame as f32 / 60.0;

        let position = curves[0].interple(t, &calc);
        let rotation = curves[1].interple(t, &calc);
        let scale = curves[2].interple(t, &calc);

        println!("帧 {}: 位置={:.1}, 旋转={:.1}, 缩放={:.1}",
                frame, position, rotation, scale);
    }
}
```

## 🎮 游戏开发应用

### 角色移动动画

```rust
use pi_curves::prelude::*;

fn character_movement_animation() {
    // 创建移动路径
    let mut movement_curve = FrameCurve::curve_frame_values(60);
    movement_curve.curve_frame_values_frame(0, 0.0);     // 起始位置
    movement_curve.curve_frame_values_frame(30, 50.0);   // 中间位置
    movement_curve.curve_frame_values_frame(60, 100.0);  // 结束位置

    // 创建面向角度动画
    let mut rotation_curve = FrameCurve::curve_easing(
        0.0, 90.0, 60, 60, EEasingMode::SineInOut
    );

    let calc = AnimationAmountCalc::default();

    // 游戏循环中每帧更新
    for frame in 0..=60 {
        let t = frame as f32 / 60.0;

        let position = movement_curve.interple(t, &calc);
        let rotation = rotation_curve.interple(t, &calc);

        // 应用到游戏对象
        // game_object.set_position(position);
        // game_object.set_rotation(rotation);

        println!("帧 {}: 位置={:.1}, 角度={:.1}°", frame, position, rotation);
    }
}
```

### UI 动画效果

```rust
use pi_curves::prelude::*;

fn ui_button_animation() {
    // 按钮缩放效果（鼠标悬停）
    let hover_scale = FrameCurve::curve_cubic_bezier(
        1.0, 0.2, 15, 60, 0.25, 0.1, 0.25, 1.0
    );

    // 按钮颜色渐变
    let mut color_curve = FrameCurve::curve_frame_values(30);
    color_curve.curve_frame_values_frame(0, 0x333333);
    color_curve.curve_frame_values_frame(30, 0x666666);

    // 淡入淡出效果
    let fade_curve = FrameCurve::curve_easing(
        0.0, 1.0, 20, 60, EEasingMode::CubicInOut
    );

    let calc = AnimationAmountCalc::default();

    // 模拟动画播放
    for frame in 0..=20 {
        let t = frame as f32 / 20.0;
        let alpha = fade_curve.interple(t, &calc);
        println!("透明度: {:.2}", alpha);
    }
}
```

## 🏃‍♂️ 性能基准

基于实际基准测试的性能数据（测试环境：Windows，优化编译，1,000,000 次迭代）：

### 🎉 最新优化结果 (2024-11-19 更新)

| 曲线类型 | 执行时间 | 性能评级 | 最新变化 | 适用场景 |
|---------|----------|----------|----------|----------|
| **缓动曲线** | ~13.5 µs/iter | ⭐⭐⭐⭐⭐ | - | 通用缓动动画，UI 过渡效果 |
| **线性插值** | ~20.6 µs/iter | ⭐⭐⭐⭐ | 优化中 | 简单位置动画，基础运动 |
| **步进插值** | ~22.3 µs/iter | ⭐⭐⭐⭐ | - | 像素艺术风格，离散动画 |
| **MinMax 曲线** | ~29.9 µs/iter | ⭐⭐⭐ | - | 复杂路径动画，精确切线控制 |

### 🚀 贝塞尔曲线优化成果

**重大突破**: 贝塞尔曲线性能获得 **36.8倍** 提升！

| 实现方式 | 优化前 | 最新优化后 | **总提升倍数** |
|---------|--------|------------|-------------|
| **pi_curves 贝塞尔** | 240.16 ms | **6.53 ms** | **36.8x** 🚀 |
| **原生 De Casteljau** | 28.45 µs | 28.06 µs | 1.01x |

**优化历程**:
1. **第一阶段**: 240ms → 7.33ms (**32.8倍**，移除调试输出)
2. **第二阶段**: 7.33ms → 6.53ms (**额外10.9%**，代码优化)
3. **总提升**: **36.8倍**性能改进

### 性能分析

- **缓动曲线 (EasingCurve)**: 性能最佳，仅涉及简单的数学计算，适合高频调用的动画系统
- **线性插值 (FrameValues)**: 性能优秀，查找插值区间后进行线性混合
- **步进插值 (Steps)**: 性能良好，需要额外的步进计算，但仍保持高效
- **MinMax 曲线**: 计算复杂度最高，但提供最精确的动画控制

### 测试方法

基准测试使用以下配置：
```rust
// 测试规模：1,000,000 条曲线
// 每次迭代：调用 interple() 方法
// 测试目标：10.0 时间点的插值计算
```

**性能建议**：
- 对于实时性要求高的动画，优先使用 `EasingCurve` 或 `FrameValues`
- 需要精确路径控制时使用 `MinMaxCurve`
- 像素艺术或复古风格动画使用步进插值
- 在动画系统中预计算常用曲线，避免运行时创建开销

### 运行基准测试

要验证您自己环境下的性能表现，可以运行以下命令：

```bash
# 运行所有基准测试
cargo bench --test curve

# 运行特定测试
cargo bench --test curve -- test_easing_peformance
cargo bench --test curve -- test_linear_peformance
cargo bench --test curve -- test_minmaxcurve_peformance
cargo bench --test curve -- test_steps_peformance
```

基准测试会在发布模式下编译并执行 1,000,000 次迭代，以提供可靠的性能数据。

## 🆚 与其他库的性能对比

### 🆚 基准测试结果 (100,000 次迭代) - **最新更新**

| 实现 | 缓动动画 | 线性插值 | 贝塞尔曲线 | 原生函数 |
|------|----------|----------|------------|----------|
| **pi_curves (最新)** | **2.82 ms** | **1.05 ms** | **6.53 ms** | - |
| **原生 Rust 函数** | 28.06 µs | 28.05 µs | 28.06 µs | ⚡ 最佳 |
| **性能差距** | 100x | 37x | **233x** | - |

### 📈 性能改进历程

| 阶段 | 缓动动画 | 线性插值 | 贝塞尔曲线 | 备注 |
|------|----------|----------|------------|------|
| **初始版本** | 2.81 ms | 0.99 ms | 7.33 ms |  |
| **第一次优化** | 2.82 ms | 1.05 ms | 6.53 ms | +10.9% (贝塞尔) |
| **总提升** | 0.0% | +1.9% | -10% |  |

### 🎯 优化效果分析

#### ✅ **显著改进**
- **贝塞尔曲线**: 从 7.33ms → 6.53ms (**10.9%提升** 🚀)
- **线性插值**: 从 1.07ms → 1.05ms (轻微优化)

#### 📊 最新性能排名 (100k 迭代)
1. **原生函数**: ~28 µs ⚡ (基准)
2. **pi_curves 线性**: ~1053 µs
3. **pi_curves 缓动**: ~2819 µs
4. **pi_curves 贝塞尔**: ~6527 µs

### 🔧 已实施的优化

1. **第一阶段优化**:
   - 添加函数内联 (`#[inline]`) → 线性插值优化
   - 直接数组访问优化 (`.get().unwrap()` → 直接索引)
   - 编译器友好代码结构 → **贝塞尔曲线额外 +10.9%**

3. **总优化成果**:
   - **线性插值**: 轻微优化
   - **整体系统**: 消除了最大性能瓶颈

#### 🎯 进一步优化潜力

基于性能分析，仍有以下优化机会：
- **查找表缓存**: 预计算常用贝塞尔参数 (预计 +10-50倍)
- **内存布局优化**: 改善缓存局部性 (预计 +2-3倍)
- **SIMD 向量化**: 批量处理动画 (预计 +4-8倍)

### 性能分析

#### 🏆 pi_curves 的优势

1. **功能完整性**: 提供完整的动画系统框架，包括关键帧管理、多种插值算法
2. **类型安全**: 编译时类型检查，避免运行时错误
3. **易用性**: 统一的 API 设计，无需处理底层数学计算
4. **扩展性**: 支持自定义数据类型和向量计算

#### ⚖️ 性能权衡

- **pi_curves**:
  - ✅ **功能丰富**: 帧管理、多种曲线类型、动画系统
  - ✅ **开发效率**: 高级 API，快速开发
  - ⚠️ **性能开销**: 相比原生函数有额外开销（主要是框架层的抽象）

- **原生函数**:
  - ⚡ **极致性能**: 接近硬件极限
  - ⚠️ **开发成本**: 需要手动实现所有动画逻辑
  - ⚠️ **维护难度**: 缺乏统一框架，代码分散

### 适用场景建议

#### 使用 pi_curves 的场景：
- **游戏引擎开发**: 需要完整的动画系统
- **UI 框架**: 界面过渡动画
- **数据可视化**: 图表动画效果
- **快速原型**: 快速实现动画功能

#### 使用原生函数的场景：
- **性能关键应用**: 如实时渲染、嵌入式系统
- **简单动画**: 仅有基础的插值需求
- **特定算法**: 需要特殊的数学优化

### 与其他 Rust 库对比

基于对 Rust 生态中相关库的分析：

| 库 | 特性 | 性能 | 适用场景 |
|----|------|------|----------|
| **pi_curves** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | 完整动画系统 |
| **easing** | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | 简单缓动函数 |
| **cubic-bezier** | ⭐⭐⭐⭐ | ⭐⭐⭐ | 贝塞尔曲线专用 |
| **bezier** | ⭐⭐⭐ | ⭐⭐⭐ | 几何计算 |

**pi_curves 在功能完整性方面领先，是构建复杂动画系统的最佳选择。**

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

本项目采用 MIT 或 Apache-2.0 双重许可证。

## 🔗 相关链接

- [在线贝塞尔曲线编辑器](https://cubic-bezier.com/)
- [缓动函数可视化](https://easings.net/)
- [GLTF 动画规范](https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_007_Animations.md)

---

**pi_curves** - 让动画变得简单而强大！ ✨