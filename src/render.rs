
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

}


impl Camera {

    pub fn create( pos : Vec3, ang : Vec3, fovx : f32, fovy : f32) -> Camera{

        Camera {pos : pos, ang : ang, fovx : fovx, fovy : fovy}

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




