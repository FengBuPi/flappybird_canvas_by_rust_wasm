// 模块化
use crate::bird::Bird;
use crate::draw::Draw;
use crate::land::Land;
use crate::pipe::Pipe;
use crate::sky::Sky;

// 用于调试代码的第三方库
use console_error_panic_hook::set_once;
use console_log::{self};
use log::Level; // 调试

// 标准库
use std::cell::RefCell;
use std::panic;
use std::rc::Rc;

// wasm 运行环境
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
        for i in 0..4 {
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

        // 先绘制所有元素
        for drawable in self.drawable_list.iter() {
            drawable.borrow_mut().draw();
        }

        if let Some(bird) = &self.bird {
            let bird_ref = bird.borrow();
            // 获取小鸟的边界
            if let Some((bird_x, bird_y, bird_width, bird_height)) = bird_ref.get_bounds() {
                // 检查与地面和天花板的碰撞
                if (bird_ref.y > (self.context.canvas().unwrap().height() - 112) as f64)
                    || (bird_ref.y < -10.0)
                {
                    log::info!("碰到地面或天花板，游戏结束！");
                    // TODO: 在这里添加游戏结束的处理逻辑
                }

                // 检查与管道的碰撞
                for drawable in self.drawable_list.iter() {
                    let drawable_ref = drawable.borrow();
                    // 尝试将drawable转换为Pipe类型
                    if let Some(pipe) = drawable_ref.as_any().downcast_ref::<Pipe>() {
                        let (pipe_x, pipe_top_y, _, pipe_bottom_y) = pipe.get_pipes_position();
                        let pipe_width = pipe.get_width();

                        // 检查与上管道的碰撞
                        if self.check_collision(
                            bird_x,
                            bird_y,
                            bird_width,
                            bird_height,
                            pipe_x,
                            pipe_top_y,
                            pipe_width,
                            pipe.get_height(),
                        ) {
                            log::info!("碰到上管道，游戏结束！");
                            // TODO: 添加游戏结束处理逻辑
                        }

                        // 检查与下管道的碰撞
                        if self.check_collision(
                            bird_x,
                            bird_y,
                            bird_width,
                            bird_height,
                            pipe_x,
                            pipe_bottom_y,
                            pipe_width,
                            pipe.get_height(),
                        ) {
                            log::info!("碰到下管道，游戏结束！");
                            // TODO: 添加游戏结束处理逻辑
                        }
                    }
                }
            }
        }
    }

    // 添加碰撞检测辅助方法
    fn check_collision(
        &self,
        x1: f64,
        y1: f64,
        w1: f64,
        h1: f64,
        x2: f64,
        y2: f64,
        w2: f64,
        h2: f64,
    ) -> bool {
        x1 < x2 + w2 && x1 + w1 > x2 && y1 < y2 + h2 && y1 + h1 > y2
    }
}
