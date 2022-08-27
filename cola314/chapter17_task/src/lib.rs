pub mod design_pattern {
    use std::rc::Weak;

    pub trait Observer<T> {
        fn update(&self, obj: &T);
    }
    
    pub struct Subject<T> {
        observers: Vec<Weak<dyn Observer<T>>>
    }
    
    impl<T> Subject<T> {
        pub fn new() -> Self {
            Subject { observers: Vec::new() }
        }
    
        pub fn register_observer(&mut self, observer: Weak<dyn Observer<T>>) {
            self.observers.push(observer);
        }
    
        pub fn notify_observer(&self, value: T) {
            for observer in self.observers.iter() {
                if let Some(observer) = observer.upgrade() {
                    observer.as_ref().update(&value);
                }
            }
        }
    }
}