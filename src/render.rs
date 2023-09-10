
pub struct Vec3 {

    x : f64,
    y : f64,
    z : f64,

}


impl Vec3 {

    pub fn create( x : f64, y : f64, z : f64) -> Vec3 {

        Vec3 { x : x, y : y, z : z}

    }

    pub fn to_array(&self) -> [f64;3] {

        [self.x, self.y, self.z]

    }

}




pub struct Camera {

    pos : Vec3,
    ang : Vec3,
    fovx : f64,
    fovy : f64,

}


impl Camera {

    pub fn create( pos : Vec3, ang : Vec3, fovx : f64, fovy : f64) -> Camera{

        Camera {pos : pos, ang : ang, fovx : fovx, fovy : fovy}

    }






}

pub struct LightRadial {

    pos : Vec3,
    color : Vec3,
    strength : f64,
    radius : f64,

}


impl LightRadial {

    pub fn create( pos : Vec3, color : Vec3, strength : f64, radius : f64) -> LightRadial {

        LightRadial {pos : pos, color : color, strength : strength, radius : radius}

    }






}




