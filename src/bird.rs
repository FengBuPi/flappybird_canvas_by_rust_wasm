use crate::draw::Draw;
use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::*;

pub struct Bird {
    ctx: CanvasRenderingContext2d,
    bird_img: HtmlImageElement,
    pub x: f64,
    pub y: f64,
    bird_width: f64,
    bird_height: f64,
    index: f64,
    v0: Rc<RefCell<f64>>,
    acc: f64,
    start_time: f64,
    max_speed: f64,
    max_angle: f64,
}

impl Bird {
    // 创建一个新的 Bird
    pub fn new(ctx: &CanvasRenderingContext2d, bird_img: HtmlImageElement) -> Bird {
        let bird_width = bird_img.width() as f64 / 3.0;
        let bird_height = bird_img.height() as f64;
        let start_time = window().unwrap().performance().unwrap().now(); // 返回当前时间的高精度时间戳

        let mut bird = Bird {
            ctx: ctx.clone(),
            bird_img,
            x: 50.0,
            y: 100.0,
            bird_width,
            bird_height,
            index: 0.0,
            v0: Rc::new(RefCell::new(0.0)),
            acc: 0.0005,
            start_time,
            max_speed: 0.3,
            max_angle: std::f64::consts::PI / 4.0,
        };

        bird.init_fly();
        bird
    }

    // 初始化飞行，绑定点击事件
    fn init_fly(&mut self) {
        let v0 = Rc::clone(&self.v0);
        let cb = Closure::wrap(Box::new(move || {
            *v0.borrow_mut() = -0.3;
            log::info!("点击了鸟,v0:{}", *v0.borrow());
        }) as Box<dyn FnMut()>);

        // 绑定点击事件监听
        self.ctx
            .canvas()
            .unwrap()
            .add_event_listener_with_callback("click", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget(); // 防止闭包被清理
    }

    pub fn get_bounds(&self) -> Option<(f64, f64, f64, f64)> {
        let width = self.bird_width * 0.8;
        let height = self.bird_height * 0.8;
        Some((self.x - width / 2.0, self.y - height / 2.0, width, height))
    }
}

// 绘制鸟
impl Draw for Bird {
    fn draw(&mut self) {
        self.ctx.save();
        self.ctx.translate(self.x, self.y).unwrap(); // 控制小鸟位移
                                                     // 时间间隔
        let delta_time = 5.0;
        // 小鸟位移 x=v0t+(a*t^2)/2
        self.y += *self.v0.borrow() * delta_time + self.acc * delta_time * delta_time / 2.0;

        let current_v0 = *self.v0.borrow(); // 不可变借用一次，缓存当前值
        *self.v0.borrow_mut() = current_v0 + self.acc * delta_time;

        // 小鸟旋转
        let angle = *self.v0.borrow() / self.max_speed * self.max_angle;
        if angle > self.max_angle {
            self.ctx.rotate(self.max_angle).unwrap();
        } else {
            self.ctx.rotate(angle).unwrap();
        }
        // 绘制鸟的图像
        self.ctx
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.bird_img,
                ((self.index / 5.0).floor() * self.bird_width).floor(),
                0.0,
                self.bird_width.floor(),
                self.bird_height.floor(),
                -(self.bird_width / 2.0).floor(),
                -(self.bird_height / 2.0).floor(),
                self.bird_width.floor(),
                self.bird_height.floor(),
            )
            .unwrap();
        // log::info!("当前self.v0: {}", *self.v0.borrow());
        self.index += 1.0;
        if self.index > 14.0 {
            self.index = 0.0;
        }
        self.ctx.restore(); // 恢复状态
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
