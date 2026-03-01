use std::io::Write;

use raylib::{self, prelude::RaylibDraw};
mod fourier_series;


fn render_square_signal(max_sums:i32){
    let width_range: i32= ( 5.0 * std::f32::consts::PI ) as i32;
    let height_range: i32 = 5;

    let mut direction:i32 = 1;

    const RECT_WIDTH :i32 = 1;
    const FPS:u32 = 100;
    const STEP:f32 = 0.01;
    const SPEED:f32 = 0.5;


    let (mut rl, thread) = raylib::init()
        .resizable()
        .transparent()
        .size(1000, 800)
        .title("square")
        .build();

    rl.set_target_fps(FPS);


    
    let mut square_signal : fourier_series::Signal = fourier_series::Signal::new(0.0, width_range, height_range);
    square_signal.set_draw_precision(RECT_WIDTH);

    let mut sums: i32 = 1;

    while ! rl.window_should_close(){
        let mut draw_handle: raylib::prelude::RaylibDrawHandle<'_> = rl.begin_drawing(&thread);
        draw_handle.clear_background(raylib::color::Color::BLANK);
        square_signal.draw_axis(&mut draw_handle);
        for x in (-(width_range as f32 / STEP)as i32) .. ((width_range as f32 / STEP)as i32){
            square_signal.set_x(x as f32 * STEP);
        
            
            square_signal.render_square(&mut draw_handle, sums);
            
        
        }
        print!("\r precision={} , max_sums={} , direction={},       [FPS={}]", sums, max_sums, direction, draw_handle.get_fps());
        sums = (sums + 1* direction);

        if sums.abs() <= 0 || sums >= max_sums{
            direction = -direction;
        }
        //std::thread::sleep(std::time::Duration::from_secs_f32(1.0/SPEED /FPS as f32));
        std::io::stdout().flush();

    }   
}


fn main() {
    render_square_signal(50);
}
