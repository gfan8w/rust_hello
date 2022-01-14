
#[cfg(feature = "bmp")]
pub mod bmp {
    ///show a bmp image, if only the feature is set to `bmp`
    pub fn show(){
        println!("this is a bmp.");
    }
}

#[cfg(feature = "png")]
pub mod png {
    ///show a png image, if only the feature is set to `png`
    pub fn show(){
        println!("this is a png.");
    }
}




