use std::time::{SystemTime, UNIX_EPOCH};
use wasm_bindgen::prelude::*;
use web_sys::*;

struct Bird {
    ctx: CanvasRenderingContext2d,
    bird_img: HtmlImageElement,
    pub x: f64,
    pub y: f64,
    bird_width: f64,
    bird_height: f64,
    index: f64,
    v0: f64,
    acc: f64,
    start_time: f64,
    max_speed: f64,
    max_angle: f64,
}

impl Bird {
    fn new(ctx: CanvasRenderingContext2d, bird_img: HtmlImageElement) -> Bird {
        let bird_width = bird_img.width() as f64 / 2.0;
        let bird_height = bird_img.height() as f64 / 2.0;
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let mut bird = Bird {
            ctx,
            bird_img,
            x: 100.0,
            y: 100.0,
            bird_width,
            bird_height,
            index: 0.0,
            v0: 0.0,
            acc: 0.0005,
            start_time,
            max_speed: 0.3,
            max_angle: 3.1415926 / 4.0,
        };
        bird.init_fly();
        bird
    }

    fn init_fly(&mut self) {
        let ctx = self.ctx.clone();
        let bird = self.clone();
        self.ctx.canvas().unwrap().set_onclick(Some(
            &Closure::wrap(Box::new(move |_| {
                bird.init_fly();
            }) as Box<dyn FnMut(_)>)
            .into_js_value(),
        ));
    }

    fn draw(&mut self) {
        self.ctx.save();
        self.ctx.translate(self.x, self.y);
        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();
        let delta_time = current_time - self.start_time;
        self.start_time = current_time;
        let h = self.v0 * delta_time + self.acc * delta_time * delta_time / 2.0;
        self.y += h;
        self.v0 += self.acc * delta_time;
        let angle = self.v0 / self.max_speed * self.max_angle;
        if angle > self.max_angle {
            self.ctx.rotate(self.max_angle);
        } else {
            self.ctx.rotate(angle);
        }
        self.ctx
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &self.bird_img,
                (self.index / 5.0).floor() * self.bird_width,
                0.0,
                self.bird_width,
                self.bird_height,
                -self.bird_width / 2.0,
                -self.bird_height / 2.0,
                self.bird_width,
                self.bird_height,
            )
            .unwrap();
        self.index += 1.0;
        if self.index > 14.0 {
            self.index = 0.0;
        }
        self.ctx.restore();
    }
}
