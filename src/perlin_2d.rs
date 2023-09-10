use rand::Rng;
use std::f64::consts::PI;

pub struct NoiseMap2D {

    octaves : Vec<[u64; 2]>,
    gradients : Vec<Vec<Vec<[f64; 2]>>>,

}

fn dot(a : &[f64 ; 2], b : &[f64 ; 2] ) -> f64 {
    a[0] * b[0] + a[1] * b[1]
}

fn smoother_step(d1 : f64, d2 : f64, w : f64) -> f64 {

    (d2 - d1) * ((w * (w * 6.0 - 15.0) + 10.0) * w * w * w) + d1
    
}

fn check_range_inclusive( x : &f64, min : &f64, max : &f64) -> bool {

    if x >= min && x <= max { true }
    else { false }

}

fn normalize( vec : [f64; 2]) -> [f64 ; 2] {

    let length = (vec[0].powi(2) + vec[1].powi(2)).sqrt();

    if length != 0.0 {
        [vec[0]/length, vec[1]/length]
    }
    else {
        [0.0, 0.0]
    }


}

impl NoiseMap2D {

    pub fn new( octave_resolution : Vec<[u64; 2]>) -> NoiseMap2D {

        // Init rng
        let mut rng = rand::thread_rng();
        let mut octaves : Vec<Vec<Vec<[f64;2]>>> = Vec::new();

        // Build Each octave
        for sector_count in &octave_resolution {

            // Make sure to include atleast one sector
            if sector_count[0] != 0 && sector_count[1] != 0 {


                // Generate 2D Gradient Array
                let mut octave : Vec<Vec<[f64; 2]>> = Vec::new();

                for _row in 0..=sector_count[1] {

                    let mut current_row : Vec<[f64; 2]> = Vec::new();

                    for _col in 0..=sector_count[0] {

                        let ang = rng.gen_range(0.0..=2.0*PI);
                        let direction: [f64 ; 2] = [ang.cos() , ang.sin()];
                        current_row.push(direction);

                    }

                    octave.push(current_row);

                }
                
                octaves.push(octave);
            }
        
        
            else {panic!("Expected atleast one sector, found dimm [{} {}]", sector_count[0], sector_count[1])}
        
        }
            NoiseMap2D { octaves : octave_resolution, gradients : octaves}


    }

    pub fn poll(&self, x : f64, y : f64) -> f64 {

        if check_range_inclusive(&x, &0.0, &1.0) && check_range_inclusive(&y, &0.0, &1.0) {

            let mut accumulated = 0.0;
            let mut i = 0;

            for sectors in &self.octaves {

                let blocksize_x = 1.0 / sectors[0] as f64;
                let blocksize_y = 1.0 / sectors[1] as f64;

                let left = (x / blocksize_x).floor().clamp(0.0, (self.octaves[i][0]-1) as f64) as usize;
                let top = (y / blocksize_y).floor().clamp(0.0, (self.octaves[i][1]-1) as f64) as usize;

                // Fetch gradient Vectors
                let tl = self.gradients[i][top][left];
                let tr = self.gradients[i][top][left+1];
                let bl = self.gradients[i][top+1][left];
                let br = self.gradients[i][top+1][left+1]; 

                // Get displacement Vectors
                let d1 : [f64; 2] = [x - (left) as f64 * blocksize_x, y - (top) as f64 * blocksize_y];
                let d2 : [f64; 2] = [x - (left+1) as f64 * blocksize_x, y - (top) as f64 * blocksize_y];
                let d3 : [f64; 2] = [x - (left) as f64 * blocksize_x, y - (top+1) as f64 * blocksize_y];
                let d4 : [f64; 2] = [x - (left+1) as f64 * blocksize_x, y - (top+1) as f64 * blocksize_y];

                // Calculate Dot products
                let dot1 = dot(&d1,&tl);
                let dot2 = dot(&d2,&tr);
                let dot3 = dot(&d3,&bl);
                let dot4 = dot(&d4,&br);

                // Interpolate between Dot Products
                let x_weight = (x - left as f64 * blocksize_x)/blocksize_x;
                let y_weight = (y - top as f64 * blocksize_y)/blocksize_y;

                

                let l1 = smoother_step(dot1, dot2, x_weight);
                let l2 = smoother_step(dot3, dot4, x_weight);
                let value = smoother_step(l1, l2, y_weight);

                let qd = ( blocksize_x.powi(2) + blocksize_y.powi(2) ).sqrt();

                accumulated += ((value / (qd / 2.0 ) + 1.0) / 2.0 ) * 0.5_f64.powf(i as f64);

                i += 1;
                
            }

            
            accumulated /= (1.0 - 0.5_f64.powf(i as f64))/0.5;
            accumulated
            

        }
        else { panic! ("Expected polled co-ordinates to be in range : 0.0 =< x <= 1.0, found : {} {} ", x, y) }
        


    }


    pub fn rotate(&mut self, scale : f64) {

        let mut new_gradients : Vec<Vec<Vec<[f64;2]>>> = Vec::new();

        for octave in &self.gradients {

            let mut new_octave: Vec<Vec<[f64;2]>> = Vec::new();

            for row in octave {


                let mut rng = rand::thread_rng();
                let offset = rng.gen_range(0.0..0.1);
                let mut new_row: Vec<[f64;2]> = Vec::new();

                for gradient in row {

                    let effective_scale = scale + offset;
                    let rx = effective_scale.cos()*gradient[0] - effective_scale.sin()*gradient[1];
                    let ry = effective_scale.sin()*gradient[0] + effective_scale.cos()*gradient[1];
                    let new_gradient = [rx , ry];
                    new_row.push(new_gradient);

                }

                new_octave.push(new_row);

            }

            new_gradients.push(new_octave)

        }

        self.gradients = new_gradients;

    }


}