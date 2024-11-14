use crate::Bird::Bird;
use console_error_panic_hook::set_once;
use console_log;
use log::Level; // 调试
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::*;
#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    bird: Option<Bird>,
}

#[wasm_bindgen]
impl Game {
    // 构造函数
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: &str) -> Game {
        console_log::init_with_level(Level::Debug).expect("初始化日志失败");
        panic::set_hook(Box::new(console_error_panic_hook::hook));
        set_once(); // wasm调试钩子
        let document = web_sys::window().unwrap().document().unwrap(); // 获取 document对象
        let canvas = document.get_element_by_id(ctx).unwrap(); // 获取 canvas对象
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();

        // 设置 Canvas 宽高
        // canvas.set_width(800);
        // canvas.set_height(600);

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Game {
            context,
            bird: None,
        }
    }

    pub fn game_start(&mut self) {
        self.init();
    }

    fn init(&mut self) {
        self.init_bird();
    }

    fn init_bird(&mut self) {
        // 加载图像
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/flappybird_Rust/asset/images/birds.png");
        self.bird = Some(Bird::new(&self.context, image));
    }

    pub fn animation(&mut self) {
        // 清除画布
        self.context.clear_rect(
            0.0,
            0.0,
            self.context.canvas().unwrap().width() as f64,
            self.context.canvas().unwrap().height() as f64,
        );

        // 开启新路径
        self.context.begin_path();

        self.bird.as_mut().expect("self.bird方法绘制错误").draw();
    }
}
