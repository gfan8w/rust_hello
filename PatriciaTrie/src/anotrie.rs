

fn run1() {

    let mut trie =Node::new();
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


pub trait trie {
    fn insert(&mut self, word: &str);
    fn search(&self, word: &str) -> bool;
    fn startWith(&self, word: &str) -> bool;
}

#[derive(PartialEq, Debug)]
pub struct Node {
    isWord: u8,
    child: Vec<Node>
}

impl Default for Node {
    fn default() -> Self {
        Self {
            isWord:0,
            child:Default::default()
        }
    }
}


impl Node {
    pub(crate) fn new() -> Self {
        Node {
            isWord:1,
            child: (0..25).map(|_|Node::default()).collect()
        }
    }
}

impl trie for Node {
    fn insert(&mut self, word: &str) {
        let mut node = self;

        for c in word.chars() {
            let i = (c as usize) - ('a' as usize);
            let ab = &node.child[i];
            //let abc = &ab.unwrap();
            //let abd = abc.unwrap().as_ref();
            if  ab==&Node::default() {
                node.child[i]=Node::new()
            }

            node = &mut node.child[i]
        }

        node.isWord=2;

    }

    fn search(&self, word: &str) -> bool {
        let mut node = self;

        for c in word.chars() {
            let i = (c as usize) - ('a' as usize);
            let ab =&node.child[i];

            if ab==&Node::default() {
                return false
            }

            node = &node.child[i]
        }

        return node.isWord==2
    }

    fn startWith(&self, word: &str) -> bool {
        let mut node = self;

        for c in word.chars() {
            let i = (c as usize) - ('a' as usize);
            let ab =&node.child[i];
            if ab==&Node::default() {
                return false
            }

            node = &node.child[i]
        }

        return true
    }
}




















