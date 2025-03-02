use crate::draw::Draw;
use js_sys::Math;
use std::any::Any;
use web_sys::*;

pub struct Pipe {
    ctx: CanvasRenderingContext2d,
    pipe1_image: HtmlImageElement,
    pipe2_image: HtmlImageElement,
    x: f64,
    speed: f64,
    random: f64,
}

impl Pipe {
    pub fn new(
        ctx: &CanvasRenderingContext2d,
        pipe1_image: HtmlImageElement,
        pipe2_image: HtmlImageElement,
        x: f64,
        speed: f64,
    ) -> Pipe {
        Pipe {
            ctx: ctx.clone(),
            pipe1_image,
            pipe2_image,
            x,
            speed,
            random: Math::random() * 200.0,
        }
    }

    pub fn get_pipes_position(&self) -> (f64, f64, f64, f64) {
        let top_y = -(self.pipe1_image.height() as f64) + self.random;
        let bottom_y = 200.0 + self.random;
        (self.x, top_y, self.x, bottom_y)
    }

    pub fn get_height(&self) -> f64 {
        self.pipe1_image.height() as f64
    }

    pub fn get_width(&self) -> f64 {
        self.pipe1_image.width() as f64
    }

    // 获取管道边界
    pub fn get_bounds(&self) -> Option<(f64, f64, f64, f64)> {
        let (_, top_y, _, _bottom_y) = self.get_pipes_position();
        let width = self.pipe1_image.width() as f64;
        let height = self.pipe1_image.height() as f64;

        Some((self.x, top_y, width, height))
    }
}

impl Draw for Pipe {
    fn draw(&mut self) {
        let (_, top_y, _, bottom_y) = self.get_pipes_position();
        let pipe1_image_width = self.pipe1_image.width() as f64;
        // 绘制管道
        self.ctx
            .draw_image_with_html_image_element(&self.pipe1_image, self.x, bottom_y)
            .unwrap();
        // 绘制管道
        self.ctx
            .draw_image_with_html_image_element(&self.pipe2_image, self.x, top_y)
            .unwrap();
        // 移动管道
        self.x -= self.speed;
        if self.x <= -pipe1_image_width {
            self.x += 18.0 * pipe1_image_width;
            self.random = js_sys::Math::random() * 200.0;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
