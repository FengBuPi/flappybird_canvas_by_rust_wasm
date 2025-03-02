use crate::draw::Draw;
use std::any::Any;
use web_sys::*;

pub struct Sky {
    ctx: CanvasRenderingContext2d,
    sky_img: HtmlImageElement,
    x: f64,
    y: f64,
    speed: f64,
}

impl Sky {
    pub fn new(
        ctx: &CanvasRenderingContext2d,
        sky_img: HtmlImageElement,
        x: f64,
        speed: f64,
    ) -> Sky {
        Sky {
            ctx: ctx.clone(),
            sky_img,
            x,
            y: 0.0,
            speed,
        }
    }
}

impl Draw for Sky {
    fn draw(&mut self) {
        self.ctx
            .draw_image_with_html_image_element(&self.sky_img, self.x, self.y)
            .unwrap();
        self.x -= self.speed;
        let sky_img_width = self.sky_img.width() as f64;
        if self.x <= -sky_img_width {
            self.x += 2.0 * sky_img_width;
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}
