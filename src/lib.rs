use wasm_bindgen::prelude::*;

struct Rng{
    seed: u64,
}

impl Rng{
    fn new(seed: u64) -> Rng{
        Rng{seed}
    }

    fn gen(&mut self, max: u64) -> u64{
        self.seed = (self.seed+1)*1664251013904223 + 1013904223;
        self.seed % max
    }
}

#[wasm_bindgen]
pub fn get_pixel_data(seed: u32) -> Vec<u8> {
    let mut rng = Rng::new(seed as u64);
    let mut data = vec![0; 400 * 400 * 4];
    for i in 0..400 {
        for j in 0..400 {
            let index = (i * 400 + j) * 4;
            data[index] = rng.gen(256) as u8;
            data[index + 1] = rng.gen(256) as u8;
            data[index + 2] = rng.gen(256) as u8;
            data[index + 3] = 255;
        }
    }
    data
}
