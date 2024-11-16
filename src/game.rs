// 模块化
use crate::bird::Bird;
use crate::draw::Draw;
// use crate::sky::Sky;

// 用于调试代码的第三方库
use console_error_panic_hook::set_once;
use console_log;
use log::Level; // 调试

// wasm 运行环境
use std::panic;
use wasm_bindgen::prelude::*;
use web_sys::*;

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

    fn init(&mut self) {
        self.init_bird();
    }

    fn init_bird(&mut self) {
        // 加载图像
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/flappybird_Rust/asset/images/birds.png");
        self.drawable_list
            .push(Box::new(Bird::new(&self.context, image)));
    }

    // fn init_sky() {
    //     // 初始化天空
    //     for _ in 0..2 {
    //         // self.drawable_list
    //         //     .push(Box::new(Sky::new(&self.context, image)));
    //         //   const sky = new Sky(this.ctx, skyImage, i * skyImage.width, this.speed);
    //         //   this.drawableList.push(sky);
    //     }
    // }

    // fn init_pipe() {
    //     // 初始化管道
    //     // const topImg = pipe2Image;
    //     // const botImg = pipe1Image;
    //     for _ in 0..6 {
    //         //   const pipe = new Pipe(this.ctx, topImg, botImg, i * 3 * topImg.width, this.speed);
    //         // self.drawable_list
    //         //     .push(Box::new(Sky::new(&self.context, image)));
    //         //   this.drawableList.push(pipe);
    //     }
    // }

    // fn init_land() {
    //     // 初始化陆地
    //     // const landImg = landImage;
    //     for _ in 0..4 {
    //         //   const land = new Land(this.ctx, landImg, i * landImg.width, this.speed);
    //         //   this.drawableList.push(land);
    //     }
    // }

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
