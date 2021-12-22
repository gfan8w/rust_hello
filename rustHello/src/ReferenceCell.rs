
///参考：https://doc.rust-lang.org/stable/book/ch15-05-interior-mutability.html
///

pub trait MessageSender {
    fn sender(&self, msg: &str);
}

pub struct LimiterTracker<'a, T:MessageSender> {
    messager: &'a T,
    value: i32,
    max: usize,
}

impl<T> LimiterTracker<'_, T> where T:MessageSender {
    pub fn new(sender: &T, max: usize) ->LimiterTracker<T> {
        LimiterTracker{
            messager:sender,
            value:0,
            max
        }
    }

    pub fn set_value(&mut self, value: i32) {
        self.value=value;

        let percent_of_warning = self.value as f32 / self.max as f32;

        if percent_of_warning >=1.00 {
            self.messager.sender("You are overflow your quota! dangerous!");
        }else if percent_of_warning >0.8 {
            self.messager.sender("You are at your 80% quota! dangerous!");
        }else if percent_of_warning>0.7 {
            self.messager.sender("You are at your 70% quota! pay an attention!");
        }else {
            self.messager.sender("You are healthy.");
        }

    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::cell::RefCell;

    struct MockMessager {
        message: RefCell<Vec<String>>,
    }

    impl MockMessager {
        fn new() -> MockMessager {
            MockMessager {
                message:RefCell::new(Vec::new())
            }
        }
    }

    impl MessageSender for MockMessager {
        fn sender(&self, msg: &str){
            self.message.borrow_mut().push(msg.to_string())
        }
    }


    #[test]
    fn it_sends_a_message_when_70_is_reach() {
        let msg_sendor = MockMessager::new();

        let mut limit= LimiterTracker::new(&msg_sendor,100 as usize);

        limit.set_value(71);

        assert_eq!(1,msg_sendor.message.borrow().len());

    }
}


pub fn run(){

}

