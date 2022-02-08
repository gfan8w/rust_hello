use std::borrow::Cow;

/// 在解析 URL 的时候，我们经常需要将 querystring 中的参数，提取成 KV pair 来进一步使用。
/// 绝大多数语言中，提取出来的 KV 都是新的字符串，在每秒钟处理几十 k 甚至上百 k 请求的系统中，
/// 你可以想象这会带来多少次堆内存的分配。
/// 但在 Rust 中，我们可以用 Cow 类型轻松高效处理它，在读取 URL 的过程中：每解析出一个 key 或者 value，
/// 我们可以用一个 &str 指向 URL 中相应的位置，然后用 Cow 封装它；而当解析出来的内容不能直接使用，
/// 需要 decode 时，比如 “hello%20world”，我们可以生成一个解析后的 String，同样用 Cow 封装它。
fn run_parse_url() {
    let link = url::Url::parse("http://www.hello.com/rust?page=1024&sort=desc&extra=hello%20world").unwrap();
    let mut pairs = link.query_pairs();
    assert_eq!(3, pairs.count());

    let (mut k, v) = pairs.next().unwrap();
    // 因为 k, v 都是 Cow 他们用起来感觉和 &str 或者 String 一样
    // 此刻，他们都是 Borrowed
    println!("key:{},value:{}", k, v);

    // 当修改发生时，k 变成 Owned
    let km = k.to_mut();
    km.push_str("_lala");

    print_pairs((k, v));

    print_pairs(pairs.next().unwrap());

    // 在处理 extra=hello%20world 时，value 被处理成 "hello world" // 所以这里 value 是 Owned
    print_pairs(pairs.next().unwrap());
}


fn print_pairs(pair: (Cow<str>, Cow<str>)) {
    show_cow(pair.0);
    show_cow(pair.1);
}

fn show_cow(val: Cow<str>) {
    match val {
        Cow::Borrowed(v) => println!("borrowed:{}",v),
        Cow::Owned(v) => println!("owned:{}",v)
    }
}











#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_run_parse_url() {
        run_parse_url()
    }

}









