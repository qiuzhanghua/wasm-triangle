# wasm-triangle

Project to draw a triangle on an HTML Canvas using Rust and WebAssembly.

一个使用 Rust 和 WebAssembly 在 HTML Canvas 上绘制三角形的项目。

## Prerequisites / 前置条件
以下软件已经安装
- Rust
- uv
- WebAssembly/binaryen installed (for `wasm-opt`)
  - Or use pack-binaryen.py script to install it
- A modern web browser that supports WebAssembly (e.g., Chrome, Firefox, Edge)

## Getting Started / 快速开始
```bash
# 为Rust添加WASM编译目标
rustup target add wasm32-unknown-unknown

# 安装 wasm-pack 工具
cargo install wasm-pack
cargo install wasm-bindgen-cli

# 创建一个名为 "wasm-triangle" 的新库项目
# cargo new --lib wasm-triangle
# cd wasm-triangle
```

### 1. Build / 构建

```bash
wasm-pack build --target web
```

### 2. Run / 运行

```bash
uvx quickhttp
```

Then open http://127.0.0.1:8000 in your browser.

然后在浏览器中打开 http://127.0.0.1:8000。

## Tech Stack / 技术栈

- Rust + `wasm-bindgen`
- WebAssembly
- HTML5 Canvas