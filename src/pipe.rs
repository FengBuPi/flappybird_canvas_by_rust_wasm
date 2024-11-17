use crate::draw::Draw;
use js_sys::Math;
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
}

impl Draw for Pipe {
    // 绘制鸟
    fn draw(&mut self) {
        let random = self.random;
        let top_y = -(self.pipe1_image.height() as f64) + random;
        let bottom_y = 200.0 + random;
        let pipe1_image_width = self.pipe1_image.width() as f64;
        let pipe1_image_height = self.pipe1_image.height() as f64;
        let pipe2_image_width = self.pipe2_image.width() as f64;
        let pipe2_image_height = self.pipe2_image.height() as f64;
        // 绘制管道1
        self.ctx
            .draw_image_with_html_image_element(&self.pipe1_image, self.x, bottom_y)
            .unwrap();
        self.ctx
            .rect(self.x, top_y, pipe1_image_width, pipe1_image_height);
        // 绘制管道2
        self.ctx
            .draw_image_with_html_image_element(&self.pipe2_image, self.x, top_y)
            .unwrap();
        self.ctx
            .rect(self.x, top_y, pipe2_image_width, pipe2_image_height);
        self.x -= self.speed;
        if self.x <= -pipe1_image_width {
            self.x += 18.0 * pipe1_image_width;
        }
    }
}
