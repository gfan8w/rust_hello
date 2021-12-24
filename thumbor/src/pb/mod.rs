use photon_rs::transform::{ SamplingFilter};
///
/// 在这个文件中，我们引入 abi.rs，并且撰写一些辅助函数。这些辅助函数主要是为了，
/// 让 ImageSpec 可以被方便地转换成字符串，或者从字符串中恢复。
///

use prost::Message;
pub use abi::*;

mod abi; //申明abi.rs, 把 abi.rs 包含到本模块中来


impl ImageSpec {
    pub fn new(specs: Vec<Spec>) -> Self {
        Self{specs}
    }
}

/// 让 ImageSpec 可以生成一个字符串
impl From<&ImageSpec> for String {
    fn from(image_spce: &ImageSpec) -> Self {
        let data = image_spce.encode_to_vec(); // encode_to_vec 用的是protocolbuf的函数
        base64::encode_config(data, base64::URL_SAFE_NO_PAD)
    }
}

/// 让 ImageSpec 可以通过一个字符串创建。比如 s.parse().unwrap()
impl TryFrom<&str> for ImageSpec {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let data = base64::decode_config(value,base64::URL_SAFE_NO_PAD)?;
        let vd=&data[..];
        Ok(ImageSpec::decode(vd)?) //decode 用的protobuf的函数
    }
}


/// 辅助函数，photon_rs 相应的方法里需要字符串
impl filter::Filter {
    pub fn to_str(&self) -> Option<&'static str> {
        match self {
            filter::Filter::Unspecified => None,
            filter::Filter::Oceanic => Some("oceanic"),
            filter::Filter::Islands =>Some("islands"),
            filter::Filter::Marine => Some("marine"),
        }
    }
}

/// 在我们定义的 SampleFilter 和 photon_rs 的 SamplingFilter 间转换
impl From<resize::SampleFilter> for SamplingFilter {
    fn from(v: resize::SampleFilter) -> Self {
        match v {
            resize::SampleFilter::CatmullRom => SamplingFilter::CatmullRom,
            resize::SampleFilter::Gaussian => SamplingFilter::Gaussian,
            resize::SampleFilter::Nearest => SamplingFilter::Nearest,
            resize::SampleFilter::Lanczos3 =>SamplingFilter::Lanczos3,
            resize::SampleFilter::Triangle =>SamplingFilter::Triangle,
            resize::SampleFilter::Undefined =>SamplingFilter::Nearest,
        }
    }
}

/// 提供一些辅助函数，让创建一个 spec 的过程简单一些
impl Spec {

    pub fn new_resize_seam_carve(width:u32, height: u32) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize{
                width,
                height,
                rtype: resize::ResizeType::SeamCarve as i32,
                filter: resize::SampleFilter::Undefined as i32,
            }))
        }
    }
    
    pub fn new_resize(width: u32, height: u32, filter: resize::SampleFilter) -> Self {
        Self {
            data: Some(spec::Data::Resize(Resize{
                width,
                height,
                rtype: resize::ResizeType::Normal as i32,
                filter: filter as i32,
            }))
        }
    }
    
    pub fn new_filter(filter: filter::Filter) -> Self {
        Self {
            data:Some(spec::Data::Filter(Filter{
                filter: filter as i32,
            }))
        }
    }

    pub fn new_watermask(x: u32, y: u32) -> Self {
        Self {
            data: Some(spec::Data::Watermark(Watermark{
                x,
                y
            }))
        }
    }


}


#[cfg(test)]
mod test {
    use std::borrow::Borrow;
    use super::*;

    #[test]
    fn encoded_spec_could_be_decoded() {
        let spec1 = Spec::new_resize(600,600,resize::SampleFilter::CatmullRom);
        let spec2 =Spec::new_filter(filter::Filter::Marine);
        let image_spec = ImageSpec::new(vec![spec1, spec2]);

        let image_spec_string: String = image_spec.borrow().into();
        assert_eq!(image_spec, image_spec_string.as_str().try_into().unwrap())
    }


}






































