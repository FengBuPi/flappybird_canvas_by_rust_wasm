// 模块化
use crate::bird::Bird;
use crate::draw::Draw;
use crate::land::Land;
use crate::pipe::Pipe;
use crate::sky::Sky;

// 用于调试代码的第三方库
use console_error_panic_hook::set_once;
use console_log;
use log::Level; // 调试

use std::cell::RefCell;
// wasm 运行环境
use std::panic;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

const GAME_SPEED: f64 = 0.5;

#[wasm_bindgen(start)]
pub fn init_wasm() {
    // wasm调试钩子
    console_log::init_with_level(Level::Debug).expect("初始化日志失败");
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    set_once();
}

#[wasm_bindgen]
pub struct Game {
    context: CanvasRenderingContext2d,
    drawable_list: Vec<Rc<RefCell<dyn Draw>>>, // 存储实现了Draw trait的实例
    bird: Option<Rc<RefCell<Bird>>>,
}

#[wasm_bindgen]
impl Game {
    // 构造函数
    #[wasm_bindgen(constructor)]
    pub fn new(ctx: &str) -> Game {
        let document = web_sys::window().unwrap().document().unwrap(); // 获取 document对象
        let canvas = document.get_element_by_id(ctx).unwrap(); // 获取 canvas对象
        let canvas: web_sys::HtmlCanvasElement = canvas
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .map_err(|_| ())
            .unwrap();
        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        Game {
            context,
            drawable_list: Vec::new(),
            bird: None,
        }
    }

    // 初始化顺序对应图层显示顺序
    pub fn game_init(&mut self) {
        self.init_sky();
        self.init_bird();
        self.init_pipe();
        self.init_land();
    }

    fn init_bird(&mut self) {
        // 加载图像
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/asset/images/birds.png");
        let bird = Bird::new(&self.context, image);
        self.bird = Some(Rc::new(RefCell::new(bird)));
        if let Some(bird) = self.bird.as_ref() {
            self.drawable_list
                .push(Rc::clone(bird) as Rc<RefCell<dyn Draw>>);
        }
    }

    // 初始化天空
    fn init_sky(&mut self) {
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/asset/images/sky.png");
        for i in 0..2 {
            let image = image.clone();
            let x = (i * image.width()) as f64;
            self.drawable_list.push(Rc::new(RefCell::new(Sky::new(
                &self.context,
                image,
                x,
                GAME_SPEED,
            ))));
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
            self.drawable_list.push(Rc::new(RefCell::new(Pipe::new(
                &self.context,
                pipe1_image,
                pipe2_image,
                x,
                GAME_SPEED,
            ))));
        }
    }

    // 初始化陆地
    fn init_land(&mut self) {
        let image = HtmlImageElement::new().unwrap();
        image.set_src("/asset/images/land.png");
        for i in 0..4 {
            let image = image.clone();
            let x = (i * image.width()) as f64;
            self.drawable_list.push(Rc::new(RefCell::new(Land::new(
                &self.context,
                image,
                x,
                GAME_SPEED,
            ))));
        }
    }

    // 动画帧
    pub fn frame(&mut self) {
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
        for drawable in self.drawable_list.iter() {
            drawable.borrow_mut().draw();
        }

        if let Some(bird) = &self.bird {
            // 碰到地面或者天花板 gameover
            if (bird.borrow().y > (self.context.canvas().unwrap().height() - 112) as f64)
                || (bird.borrow().y < -10.0)
            {
                // log::info!(
                //     "游戏结束bird.borrow().y:{},canvas height():{}",
                //     bird.borrow().y,
                //     self.context.canvas().unwrap().height()
                // );
            }

            // 碰到管道
            if self
                .context
                .is_point_in_path_with_f64(bird.borrow().x, bird.borrow().y)
            {
                // log::info!("碰到管道");
            }
            log::info!("鸟的位置x:{},y:{}", bird.borrow().x, bird.borrow().y);
        }
    }
}
