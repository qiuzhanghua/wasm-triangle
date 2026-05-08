// 导入wasm-bindgen的所有预备宏和类型
use wasm_bindgen::prelude::*;

// 当在调试模式下构建时，启用一个更友好的panic错误提示（辅助调试）
#[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    // 确保在wasm模块中，console.error是可用的
    pub fn log(s: &str);
}

// #[wasm_bindgen]宏告诉Rust，这个函数需要被暴露给JavaScript调用
#[wasm_bindgen]
#[allow(deprecated)]
// start 是我们在HTML中调用的入口函数
pub fn start() -> Result<(), JsValue> {
    // 获取window对象
    let window = web_sys::window().expect("global window does not exist");
    // 获取document对象
    let document = window.document().expect("should have a document on window");
    // 通过ID获取canvas元素
    let canvas = document.get_element_by_id("canvas")
        .expect("canvas element with id 'canvas' not found")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Element is not a canvas");

    // 获取canvas的2D渲染上下文
    let context = canvas
        .get_context("2d")
        .expect("Failed to get 2d context")
        .expect("2d context is null")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Failed to cast context to CanvasRenderingContext2d");

    // 清除画布背景（可选操作，将画布背景设为黑色）
    context.set_fill_style(&"black".into());
    context.fill_rect(0.0, 0.0, canvas.width() as f64, canvas.height() as f64);

    // 绘制三角形
    // 绘制路径的开始
    context.begin_path();

    // 定义三角形的三个顶点，这里绘制一个位于画布中央的三角形（假设画布为600x600）
    let center_x = canvas.width() as f64 / 2.0;
    let top_y = canvas.height() as f64 / 4.0;
    let bottom_y = canvas.height() as f64 * 3.0 / 4.0;

    // 移动到第一个顶点（上顶点）
    context.move_to(center_x, top_y);
    // 画线到第二个顶点（左下）
    context.line_to(canvas.width() as f64 / 4.0, bottom_y);
    // 画线到第三个顶点（右下）
    context.line_to(canvas.width() as f64 * 3.0 / 4.0, bottom_y);

    // 闭合路径，从右下顶点画回上顶点
    context.close_path();

    // 设置填充颜色为橙色，并执行填充操作
    context.set_fill_style(&"orange".into());
    context.fill();

    // 如果需要描边，可以取消下面的注释
    context.set_stroke_style(&JsValue::from_str("black"));
    context.stroke();

    Ok(())
}