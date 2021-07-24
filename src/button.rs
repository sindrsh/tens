use crate::prelude::*;

pub struct ChangeBtn {
	color: Color,
	vertex_a: Point<f32>,
	vertex_b: Point<f32>,
	vertex_c: Point<f32>,
}


impl ChangeBtn{
	
	pub fn new(up: bool, x: f32, y: f32, width: f32, height: f32) -> Self {
		let a = Point::new(x, y);
		let b = Point::new(width, 0.0) + a;
		let mut c = Point::new(width/2., height) + a;
		if up {
			c = Point::new(width/2., -height)+a;		
		}
		ChangeBtn{
			color: GRAY,
			vertex_a: a,
			vertex_b: b,
			vertex_c: c,
		}
	}

	pub fn render(&self) {
		let a = Vec2::new(self.vertex_a.x(),self.vertex_a.y());
		let b = Vec2::new(self.vertex_b.x(),self.vertex_b.y());
		let c = Vec2::new(self.vertex_c.x(),self.vertex_c.y());
		
		draw_triangle(a, b, c, self.color);	
	}
	
	pub fn update(&mut self, color: Color) {
		self.color = color;		
	}
	
	// x: mx: mouse x, my: mouse_y  
	pub fn check_mouse(&self) -> bool {
		let vec = Vec::new();
		let triangle_line = LineString(vec![self.vertex_a, self.vertex_b, self.vertex_c, self.vertex_a]);
		let triangle = Polygon(triangle_line, vec);
		let mouse = Point::new(mouse_position().0 as f32, mouse_position().1 as f32);
		triangle.contains(&mouse)
	}
}
