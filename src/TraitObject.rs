


trait Draw {
    fn draw(&self);
}

struct Button {
    pub width: i32,
    pub height: i32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        // todo:
        println!("Button is draw")
    }
}

struct SelectBox {
    pub width: i32,
    pub height: i32,
    pub options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        // todo:
        println!("SelectBox is draw")
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run (&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}


pub fn run() {

    let screen =Screen {
        components:vec![
            Box::new(Button {
                width:10,height:15,label:"click".to_string()
            }),
            Box::new(SelectBox {
                width:20,height:36,options:vec!["Country".to_string(),"Province".to_string()]
            })
        ]
    };

    screen.run();
    //blog, OOP的设计方法
    crate::blog::post::run();

}

