
mod anotrie;


/// 实现前缀树 ，参考：https://leetcode-cn.com/problems/implement-trie-prefix-tree/solution/rust-leetcode-da-an-ji-he-by-yuuang-xlhl/
///
fn main() {

    run2()
}


fn run2(){

    let mut trie =trie2::Node::new();
    trie.insert("apple");
    let t = trie.search("apple");
    println!("search `apple`: {}",t);

    let t = trie.search("bc");
    println!("search `bc`: {}",t);

    let t = trie.startWith("b");
    println!("startWith `b`: {}",t);

    let t = trie.startWith("bc");
    println!("startWith `bc`: {}",t);

    let t = trie.search("app");
    println!("search `app`: {}",t);

    let t = trie.startWith("app");
    println!("startWith `app`: {}",t);

    trie.insert("app");
    let t = trie.search("app");
    println!("search `app`: {}",t);


}

 mod trie2 {
    use std::hint;

    pub(crate) struct Node {
        isWord: bool,
        child: [Option<Box<Node>>;26]
    }

    impl Node {
        pub(crate)  fn new() -> Self {
            Node {
                isWord:false,
                child:Default::default()
            }
        }

        pub(crate) fn insert(&mut self, words: &str) {
            let mut node =self;
            let chars = words.chars();
            for char in chars {
                let i = (char as u8 -b'a') as usize;

                if node.child[i].is_none() {
                    node.child[i]=Some(Box::new(Node::new()));
                }

                let  c = &mut node.child[i];

                //let d =c.get_or_insert(Box::new(Node::new()));

                node =match c {
                    Some(v) =>v,
                    _ => unsafe { hint::unreachable_unchecked() }
                };


                println!("child len:{}",node.child.len());

            }

            node.isWord=true;
        }


        pub(crate) fn search(&self, words: &str) -> bool {
            let mut node =self;
            let chars = words.chars();
            for char in chars {
                let i = (char as u8 -b'a') as usize;
                if node.child[i].as_ref().is_none() {
                    return false;
                }
                let ac=node.child[i].as_ref();
                let ab =ac.unwrap();
                node = ab;
            }

            return node.isWord;
        }


        pub(crate) fn startWith(&self, words: &str) -> bool {
            let mut node =self;
            let chars = words.chars();
            for char in chars {
                let i = (char as u8 -b'a') as usize;
                if node.child[i].as_ref().is_none() {
                    return false;
                }
                let ac=node.child[i].as_ref();
                let ab =ac.unwrap();
                node = ab;
            }

            return true;
        }




    }



}

















