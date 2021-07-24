mod value;
mod elements;
mod button;

mod prelude{
	pub use macroquad::prelude::*;
	pub use geo::{Point, LineString, Polygon};
	pub use geo::algorithm::contains::Contains;
	pub use std::{thread, time};
	pub use crate::value::*;
	pub use crate::elements::*;
	pub use crate::button::*;
}
use prelude::*;


fn window_conf() -> Conf {
    Conf {
        window_title: "Plassverdisystemet".to_owned(),
        window_width: 1600,
        window_height: 1200,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
	
	// parameters
	let num_y = 100.0;
	let up_btn_y = num_y+160.0;	
	let down_btn_y = num_y+180.0;	
	let ch_btn_width: f32 = 80.0;
	let ch_btn_height: f32 = 30.0; 
	let ch_col = YELLOW; 
	let btn_col = GRAY;	
	let x1 = 800.0;
	let x10 = 700.0;
	let x100 = 600.0;
	let dx = 4.0;
		
	// make buttons
	let mut up_btn1 = ChangeBtn::new(true, x1+dx, up_btn_y, ch_btn_width, ch_btn_height);
	let mut up_btn10 = ChangeBtn::new(true, x10+dx, up_btn_y, ch_btn_width, ch_btn_height);
	let mut up_btn100 = ChangeBtn::new(true, x100+dx, up_btn_y, ch_btn_width, ch_btn_height);
	
	let mut down_btn1 = ChangeBtn::new(false, x1+dx, down_btn_y, ch_btn_width, ch_btn_height);
	let mut down_btn10 = ChangeBtn::new(false, x10+dx, down_btn_y, ch_btn_width, ch_btn_height);
	let mut down_btn100 = ChangeBtn::new(false, x100+dx, down_btn_y, ch_btn_width, ch_btn_height);
	
	// ones	
	let fig1_list = figs1();	
	let num1_list = nums1();
	let mut val1 = Value::new(fig1_list, num1_list, x1, num_y, 200.0, 400.0);
	
	// tens
	let fig10_list = figs10();	
	let num10_list = nums10();
	let mut val10 = Value::new(fig10_list, num10_list, x10, num_y, 200.0, 500.0);	
	
	// hundreds 
	let fig100_list = figs100();	
	let num100_list = nums100();
	let mut val100 = Value::new(fig100_list, num100_list, x100, num_y, 500.0, 400.0);	   
    
    loop {
        clear_background(WHITE);
		thread::sleep(time::Duration::from_millis(50));
		
		val1.render();
		val10.render();
		val100.render();
		
		up_btn1.render();
		up_btn1.update(btn_col);
		up_btn10.render();
		up_btn10.update(btn_col);
		up_btn100.render();
		up_btn100.update(btn_col);
		
		down_btn1.render();
		down_btn1.update(btn_col);
		down_btn10.render();
		down_btn10.update(btn_col);
		down_btn100.render();
		down_btn100.update(btn_col);
		
		if up_btn1.check_mouse() && is_mouse_button_pressed(MouseButton::Left){
			up_btn1.update(ch_col);
			if val1.get_n()!=9 || val10.get_n() 
				!=9 || val100.get_n() !=9 {  
				val1.update(true);
				}
			if val1.get_n() == 0 {
				val10.update(true);
				if val10.get_n() == 0 {
					val100.update(true);
			}
			}
		}
		
		if up_btn10.check_mouse() && is_mouse_button_pressed(MouseButton::Left){
			up_btn10.update(ch_col);
			if val10.get_n() != 9 || val100.get_n() != 9 {
				val10.update(true);
				}
			if val10.get_n() == 0 && val100.get_n() != 9 {
				val100.update(true);
			}
		}
		
		if up_btn100.check_mouse() && is_mouse_button_pressed(MouseButton::Left) {
			up_btn100.update(ch_col);
			if val100.get_n() != 9 {
				val100.update(true);
			}
		}
		
		if down_btn1.check_mouse() && is_mouse_button_pressed(MouseButton::Left){
			down_btn1.update(ch_col);
			if val1.get_n() != 0 {
				val1.update(false);
			}
			else {
				if val10.get_n() != 0 {
					val10.update(false);
					val1.update(false);			
				}
				else if val100.get_n() != 0 {
					val1.update(false);
					val10.update(false);
					val100.update(false);								
				}
				
			}
			
		}
		
		if down_btn10.check_mouse() && is_mouse_button_pressed(MouseButton::Left){
			down_btn10.update(ch_col);
			if val10.get_n() != 0 {
				val10.update(false);
			}
			else if val100.get_n() != 0 {
				val10.update(false);
				val100.update(false);
			}
		}
		
		if down_btn100.check_mouse() && is_mouse_button_pressed(MouseButton::Left) {
			down_btn100.update(ch_col);
			if val100.get_n() != 0{
				val100.update(false);
			}
		}
		
		//break;
        next_frame().await
    }
}
