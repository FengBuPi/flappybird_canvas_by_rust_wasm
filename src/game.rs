// 模块化
use crate::draw::Draw;
use crate::land::Land;
use crate::pipe::Pipe;
use crate::sky::Sky;
use crate::Bird::Bird;

// 用于调试代码的第三方库
use console_error_panic_hook::set_once;
use console_log;
use log::Level; // 调试

// wasm 运行环境
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::*;

const GAME_SPEED: f64 = 0.5;

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    drawable_list: Vec<Box<dyn Draw>>, // 存储实现了Draw trait的实例
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
            drawable_list: Vec::new(),
        }
    }

    pub fn game_start(&mut self) {
        self.init();
    }

    // 初始化顺序对应图层显示顺序
    fn init(&mut self) {
        self.init_sky();
        self.init_pipe();
        self.init_land();
        self.init_bird();
    }

    fn init_bird(&mut self) {
        // 加载图像
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/asset/images/birds.png");
        self.drawable_list
            .push(Box::new(Bird::new(&self.context, image)));
    }

    // 初始化天空
    fn init_sky(&mut self) {
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/asset/images/sky.png");
        for i in 0..2 {
            let image = image.clone();
            let x = (i * image.width()) as f64;
            self.drawable_list
                .push(Box::new(Sky::new(&self.context, image, x, GAME_SPEED)));
        }
    }

    // 初始化管道
    fn init_pipe(&mut self) {
        let pipe1_image = HtmlImageElement::new().unwrap();
        pipe1_image.set_src("/asset/images/pipe1.png");
        let pipe2_image = HtmlImageElement::new().unwrap();
        pipe2_image.set_src("/asset/images/pipe2.png");
        for i in 0..6 {
            let pipe1_image = pipe1_image.clone();
            let pipe2_image = pipe2_image.clone();
            let x = (i * 3 * pipe1_image.width()) as f64;
            self.drawable_list.push(Box::new(Pipe::new(
                &self.context,
                pipe1_image,
                pipe2_image,
                x,
                GAME_SPEED,
            )));
        }
    }

    // 初始化陆地
    fn init_land(&mut self) {
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/asset/images/land.png");
        for i in 0..4 {
            let image = image.clone();
            let x = (i * image.width()) as f64;
            self.drawable_list
                .push(Box::new(Land::new(&self.context, image, x, GAME_SPEED)));
        }
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
        // 绘制所有元素
        for drawable in self.drawable_list.iter_mut() {
            drawable.draw();
        }
    }
}
