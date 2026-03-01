use std::{io::Write, iter::Sum};

use raylib::{RaylibHandle, RaylibThread, prelude::RaylibDraw};


pub struct Signal{
    x: f32,
    y: f32,
    width_range: i32,
    height_range: i32,
    draw_precision: i32
}

impl Signal{


    pub fn new(x: f32, width_range:i32, height_range:i32)->Signal{
        Signal { x : x, y: 0.0, width_range: width_range, height_range, draw_precision: 2}
    }

    pub fn set_x(&mut self, x:f32)->&Signal{
        self.x = x;
        self
    }

    pub fn get_x(&self)->f32{
        self.x
    }

    pub fn set_draw_precision(&mut self, size : i32)->&Signal{
        self.draw_precision = size;
        self
    }



    pub fn render_square(&mut self, draw_handle: &mut raylib::prelude::RaylibDrawHandle<'_>, sums: i32 ){

        let a0 = self.x.sin();
        self.y = 0.0;
    

        for i in (1..sums).step_by(2){
            self.y += (i as f32 * self.x).sin() / i as f32;
        }
        
        

        let (x_w, y_h) = ortho_to_screen(self.x / self.width_range as f32, self.y / self.height_range as f32, &draw_handle);        
        

        draw_handle.draw_rectangle(x_w, y_h,self.draw_precision, self.draw_precision, raylib::color::Color::WHITE);    
    }


    pub fn draw_axis(&mut self, draw_handle: &mut raylib::prelude::RaylibDrawHandle<'_>){
        
        let x_left = ortho_to_screen(-1.0 *self.width_range as f32, 0.0, draw_handle);
        let x_right = ortho_to_screen(1.0 *self.width_range as f32, 0.0, draw_handle);
        let y_top = ortho_to_screen(0.0  as f32, 1.0 * self.height_range as f32, draw_handle);
        let y_bottom = ortho_to_screen(0.0  as f32, -1.0 * self.height_range as f32, draw_handle);
        
        draw_handle.draw_line(x_left.0, x_left.1, x_right.0, x_right.1, raylib::color::Color::RED );
        draw_handle.draw_line(y_top.0, y_top.1, y_bottom.0, y_bottom.1, raylib::color::Color::BLUE);
    }



}









fn ortho_to_screen(x:f32, y:f32, d: & raylib::prelude::RaylibDrawHandle<'_>)-> (i32, i32){
    let width:f32 = d.get_screen_width() as f32;
    let height:f32 = d.get_screen_height() as f32;
    
    let scr_x: f32 = (width/2.0) + x * (width/2.0);
        let scr_y: f32 = (height/2.0) + y * (height/2.0) * -1.0;
    
    //println!("   from    ({},{})",x, y);
    //println!("  to      ({},{})", scr_x, scr_y);
        return (scr_x as i32, scr_y as i32);
}
