// use echo;
// use chat;
// use proxy;

fn main() {
    // echo::start_server();
    // chat::start_server();
    // proxy::start_server();
    let cat = Cat;
    let dog = Dog;
    sound_off(&cat);
    sound_off(&dog);
}

trait Animal {
    fn make_sound(&self);
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn make_sound(&self) {
        println!("Meow ...");
    }
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof ...");
    }
}

fn sound_off(animal: &dyn Animal) {
    animal.make_sound();
}
