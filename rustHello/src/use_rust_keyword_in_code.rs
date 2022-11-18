

// 如何在字段定义中使用Rust的保留关键字： r#
// r#type
pub struct Planet {
    pub r#type: PlanetType,
}

pub enum PlanetType {
    TerrestrialPlanet,
    GasGiant,
    IceGiant,
    DwarfPlanet,
}

pub fn run(){
}


