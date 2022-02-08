use hex_slice::AsHex;

fn main() {
    println!("如何把一个 字符串显示为一个 hex的utf8 字符串格式");
}



#[test]
fn test_run_box_utf8() {
    let s ="Example.transfer";
    let b=<&str as AsRef<str>>::as_ref(&s).as_bytes();
    let s_hex = b.iter().map(|x|format!("{:02x}",x)).collect::<Vec<_>>().join("");
    println!("0x{}",s_hex);

    let s_hex = s.as_bytes().as_hex();
    println!("{:02x}",s_hex)
}