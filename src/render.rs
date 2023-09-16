use crate::perlin_2d::{self, NoiseMap2D};
pub struct Vec3 {

    pub x : f32,
    pub y : f32,
    pub z : f32,

}


impl Vec3 {

    pub fn create( x : f32, y : f32, z : f32) -> Vec3 {

        Vec3 { x : x, y : y, z : z}

    }

    pub fn to_array(&self) -> [f32;3] {

        [self.x, self.y, self.z]

    }

}




pub struct Camera {

    pub pos : Vec3,
    pub ang : Vec3,
    pub fovx : f32,
    pub fovy : f32,
    pub current_chunk : [i32;2]

}


impl Camera {

    pub fn create( pos : Vec3, ang : Vec3, fovx : f32, fovy : f32) -> Camera{

        Camera {pos : pos, ang : ang, fovx : fovx, fovy : fovy, current_chunk : [0,0]}

    }
}



pub struct LightRadial {

    pub pos : Vec3,
    pub color : Vec3,
    pub strength : f32,
    pub radius : f32,

}


impl LightRadial {

    pub fn create( pos : Vec3, color : Vec3, strength : f32, radius : f32) -> LightRadial {

        LightRadial {pos : pos, color : color, strength : strength, radius : radius}

    }

}


pub struct ChunkLoader {

    pub max_chunks : u32,
    pub loaded_chunks : Vec<Vec<Chunk>>,


}


impl ChunkLoader {

    pub fn create( noise_map : &perlin_2d::NoiseMap2D, camera : &Camera) -> ChunkLoader {

        let max_chunks = 625;
        let mut loaded_chunks: Vec<Vec<Chunk>> = Vec::new();

        // assume 0,0 spawn
        for x in -12..13  {

            let mut row : Vec<Chunk> = Vec::new();

            for z in -12..13 as i32 {

                row.push(Chunk::create([x,z], noise_map))
                
            }

            loaded_chunks.push(row);


        }
        

        ChunkLoader { max_chunks: max_chunks as u32, loaded_chunks :  loaded_chunks }

    }

    pub fn get_camera_chunk(&self, camera : &Camera) -> [i32;2] {
        [(camera.pos.x / 20.0).floor() as i32, (camera.pos.z / 20.0).floor() as i32]
    }

    pub fn shift(&mut self, shift : [i32;2], noise_map : &perlin_2d::NoiseMap2D) {

        let vertical_sign = shift[1].signum();
        let horizontal_sign = shift[0].signum();

        let vertical_shift = shift[1].abs();
        let horizontal_shift = shift[0].abs();

        for x in 0..vertical_shift{

            if vertical_sign < 0{

                //DOWN
                // Pop Chunk off each row and push a chunk to the beginning
                for i in 0..self.loaded_chunks.len(){
                    self.loaded_chunks[i].pop();
                }

                for i in 0..self.loaded_chunks.len(){
                    let left = self.loaded_chunks[i][0].coords;
                    self.loaded_chunks[i].reverse();
                    self.loaded_chunks[i].push(Chunk::create([left[0], left[1]-1], noise_map));
                    self.loaded_chunks[i].reverse();
                }
                


            }

            if vertical_sign > 0{

                //UP
                // Pop Chunk off each row and push a chunk to the beginning
                for i in 0..self.loaded_chunks.len(){
                    self.loaded_chunks[i].reverse();
                    self.loaded_chunks[i].pop();
                    self.loaded_chunks[i].reverse();
                }

                for i in 0..self.loaded_chunks.len(){
                    let left = self.loaded_chunks[i][self.loaded_chunks[i].len()-1].coords;
                    self.loaded_chunks[i].push(Chunk::create([left[0], left[1]+1], noise_map));

                }

            }
        }

        for x in 0..horizontal_shift {


            if horizontal_sign < 0 {

                // LEFT

                self.loaded_chunks.pop();

                let mut new_row : Vec<Chunk> = Vec::new();
                let first = self.loaded_chunks[0][0].coords;

                for i in 0..25 {
                    new_row.push(Chunk::create([first[0]-1,first[1]+i], noise_map))

                }

                self.loaded_chunks.reverse();
                self.loaded_chunks.push(new_row);
                self.loaded_chunks.reverse();

            }

            if horizontal_sign > 0{

                // RIGHT

                self.loaded_chunks.reverse();
                self.loaded_chunks.pop();
                self.loaded_chunks.reverse();

                let mut new_row : Vec<Chunk> = Vec::new();
                let first = self.loaded_chunks[self.loaded_chunks.len()-1][0].coords;

                for i in 0..25 {
                    new_row.push(Chunk::create([first[0]+1,first[1]+i], noise_map))

                }

                
                self.loaded_chunks.push(new_row);


            }   

        }

    }

    pub fn get_block_data(&self, height_map : &perlin_2d::NoiseMap2D) -> Vec<[f32;1200]>{

        let mut block_data : Vec<[f32; 1200]> = Vec::new();

        for i in 0..self.loaded_chunks.len() {
            for j in 0..self.loaded_chunks[i].len(){
                block_data.push(self.loaded_chunks[i][j].blocks);
            }
            
        }

        block_data

    }


}


pub struct Chunk {

    pub pos : [f32;2],
    pub coords : [i32;2],
    pub blocks : [f32; 1200],

}

impl Chunk {

    pub fn create( coords : [i32;2], noise_map : &NoiseMap2D) -> Chunk {

        let size : [u32;2] = [20,20];
        let pos : [f32;2] = [(coords[0]*20) as f32, (coords[1]*20) as f32];
        let mut blocks = [0.0; 1200];
        let mut index = 0;

        for i in 0..20 {
            for j in 0..20{

                let x = pos[0] + j as f32;
                let z = pos[1] + i as f32;
                let mut y = noise_map.poll(((x+5000.0)/10000.0) as f64, ((z+5000.0)/10000.0) as f64) as f32 * 100.0-110.0;
                if y < -60.0 {y=-60.0}
                

                // X
                blocks[index] = x;
                
                // Y
                blocks[index+1] = y.round();

                // Z
                blocks[index+2] = z;
                

                index += 3;
            }
        }
        




        Chunk { pos: pos, coords: coords, blocks : blocks }

    }

    pub fn copy( &self ) -> Chunk {
        Chunk { pos: self.pos, coords: self.coords, blocks : self.blocks}
    }






}




