use std::rc::Rc;
use chapter17_task::design_pattern::{Observer, Subject};

struct ConsoleWriterObserver {
    name: String
}

impl ConsoleWriterObserver {
    fn new(name: String) -> Self {
        ConsoleWriterObserver {
            name
        }
    }
}

impl Observer<String> for ConsoleWriterObserver  {
    fn update(&self, obj: &String) {
        println!("[{}]: {}", self.name, obj);
    }
}

fn main() {
    let mut subject = Subject::new();

    let observer1: Rc<dyn Observer<String>> = Rc::new(ConsoleWriterObserver::new("observer1".to_string()));
    subject.register_observer(Rc::downgrade(&observer1));

    subject.notify_observer("A".to_string());

    {
        let observer2: Rc<dyn Observer<String>> = Rc::new(ConsoleWriterObserver::new("observer2".to_string()));
        subject.register_observer(Rc::downgrade(&observer2));

        subject.notify_observer("B".to_string());
    }

    subject.notify_observer("C".to_string());
}
