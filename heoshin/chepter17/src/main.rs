
pub struct Invoker {
    pub command_list: Vec<Box<dyn AnimalCommand>>,
}
impl Invoker {
    pub fn add_command(&mut self, command: Box<dyn AnimalCommand>) {
        self.command_list.push(command);
    }
    pub fn run_command(&self) {
        for command in self.command_list.iter() {
            command.excute();
        }
    }

    fn new() -> Self {
        Invoker {
            command_list: vec![],
        }
    }
}
pub trait AnimalCommand {
    fn excute(&self);
}
pub struct DogCommand<'a> {
    name: &'a str,
    age: u32,
}
impl AnimalCommand for DogCommand<'_> {
    fn excute(&self) {
        println!("Hello, My name is {}!, age is {}", self.name, self.age);
    }
}
pub struct CatCommand<'a> {
    name: &'a str,
    age: u32,
}
impl AnimalCommand for CatCommand<'_> {
    fn excute(&self) {
        println!("Hello, My name is {}!, age is {}", self.name, self.age);
    }
}

fn main() {
    let dog_command = Box::new(DogCommand{ name: "puppy", age: 5 });
    let cat_command = Box::new(CatCommand{ name: "naby", age: 4 });
    let dog_command2 = Box::new(DogCommand{ name: "choco", age: 3 });

    let mut invoker = Invoker::new();
    invoker.add_command(dog_command);
    invoker.add_command(cat_command);
    invoker.add_command(dog_command2);

    invoker.run_command();
}
