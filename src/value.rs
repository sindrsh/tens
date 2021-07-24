use crate::prelude::*;

pub struct Value {
	n: usize,
	fig_list: [Texture2D; 9],
	num_list: [Texture2D; 10],
	x: f32,
	y: f32,
	fig_x: f32,
	fig_y: f32,
}

impl Value{
	pub fn new(flist: [Texture2D; 9], list: [Texture2D; 10], x: f32, y: f32, fx: f32, fy: f32) -> Self {
		Value {
			n: 0,
			fig_list: flist,
			num_list: list,
    		x: x,
    		y: y,
    		fig_x: fx,
    		fig_y: fy,
		}
	}
	
	pub fn render(&self){
		draw_texture(self.num_list[self.n], self.x, self.y,WHITE);
		if self.n != 0 {
			//let n = self.n as f32;
			draw_texture(self.fig_list[self.n-1], self.fig_x, self.fig_y,WHITE); 
		}		
	}
	
	pub fn update(&mut self, up: bool){
		if up {
			if self.n < 9 { self.n = self.n+1; }
			else { self.n = 0; }	
		}
		else {
			if self.n > 0 { self.n = self.n-1; }
			else { self.n = 9; }
		}
	}
	
	pub fn get_n(&self) -> i32 {
		self.n as i32
	}	
}
