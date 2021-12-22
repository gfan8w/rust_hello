
#[cfg(feature = "bmp")]
pub mod bmp {
    pub fn show(){
        println!("this is a bmp.");
    }
}

#[cfg(feature = "png")]
pub mod png {
    pub fn show(){
        println!("this is a png.");
    }
}




