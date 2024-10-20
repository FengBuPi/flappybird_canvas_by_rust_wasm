use wasm_bindgen::prelude::*;
use web_sys::*;
use std::time::{SystemTime, UNIX_EPOCH};

struct Bird {
  ctx: CanvasRenderingContext2d,
  bird_img: HTMLImageElement,
  pub x: f64,
  pub y: f64,
  bird_width: f64,
  bird_height: f64,
  index: f64,
  v0: f64,
  acc: f64,
  start_time: f64,
  max_speed: f64,
  max_angle: f64
}

impl Bird {
    fn new(ctx: CanvasRenderingContext2d,bird_img: HTMLImageElement) -> Bird {
      let bird_width = bird_img.width / 2;
      let bird_height = bird_img.height / 2;
      let start_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs_f64();
      let bird =  Bird {
          ctx,
          bird_img,
          x:100.0,
          y:100.0,
          bird_width,
          bird_height,
          index:0.0,
          v0:0.0,
          acc:0.0005,
          start_time,
          max_speed: 0.3,
          max_angle: 3.1415926 / 4,
      };
      bird.init_fly();
      bird
    }

    init_fly(&mut self){
      
    }

    fn draw(&mut self) {
      self.ctx.canvas().unwrap().onClick = move |_| {
          self.init_fly();
      };
    }
}
