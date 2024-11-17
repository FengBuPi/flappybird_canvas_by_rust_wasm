use crate::draw::Draw;
use web_sys::*;

pub struct Land {
    ctx: CanvasRenderingContext2d,
    land_img: HtmlImageElement,
    x: f64,
    y: f64,
    speed: f64,
}

impl Land {
    pub fn new(
        ctx: &CanvasRenderingContext2d,
        land_img: HtmlImageElement,
        x: f64,
        speed: f64,
    ) -> Land {
        let land_img_height = land_img.height() as f64;
        Land {
            ctx: ctx.clone(),
            land_img,
            x,
            y: ctx.canvas().unwrap().height() as f64 - land_img_height,
            speed,
        }
    }
}

impl Draw for Land {
    // 绘制鸟
    fn draw(&mut self) {
        self.ctx
            .draw_image_with_html_image_element(&self.land_img, self.x, self.y)
            .unwrap();
        self.x -= self.speed;
        let land_img_width = self.land_img.width() as f64;
        if self.x <= -land_img_width {
            self.x += 4.0 * land_img_width;
        }
    }
}
