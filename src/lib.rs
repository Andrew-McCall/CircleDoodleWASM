use wasm_bindgen::prelude::*;



struct Circle{
    pub x:u32,
    pub y:u32,
    pub r:u32,
    pub r2:u32,
}

impl Circle{
    pub fn new(x:u32, y:u32, r:u32) -> Circle{
        Circle{x,y,r, r2:r*r}
    }
}

static mut CIRCLES: Vec<Circle> = Vec::new();
    
fn count_circles(x:usize, y:usize) -> usize{
    let mut count = 0;
    unsafe {
        for c in CIRCLES.iter(){
            let dx = c.x as usize - x;
            let dy = c.y as usize - y;
            if dx*dx + dy*dy < c.r2 as usize{
                count += 1;
            }
        }
    }
    count
}
const OFF: (u8,u8,u8) = (255, 255, 255);
const ON: (u8,u8,u8) = (160, 10, 220);
const SIZE: usize = 600;

#[wasm_bindgen]
pub fn get_pixel_data() -> Vec<u8> {
    let mut data = vec![0; SIZE * SIZE * 4];

    for i in 0..SIZE {
        for j in 0..SIZE {
            let index = (i * SIZE + j) * 4 ;
            if count_circles(j,i) % 2 == 0 {
                data[index] = OFF.0;
                data[index + 1] = OFF.1;
                data[index + 2] = OFF.2;
                data[index + 3] = 255;
            } else {
                data[index] = ON.0;
                data[index + 1] = ON.1;
                data[index + 2] = ON.2;
                data[index + 3] = 255;
            }
        }
    }


  
    data
}

#[wasm_bindgen]
pub fn add_circle(x:u32, y:u32, r:u32){
    unsafe {
        CIRCLES.push(Circle::new(x,y,r));
    }
}
