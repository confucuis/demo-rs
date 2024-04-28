// use echo;
// use chat;
// use proxy;

fn main() {
    // echo::start_server();
    // chat::start_server();
    // proxy::start_server();
    let p1 = Person { name: "zhangsan" };
    let p2 = Person { name: "wangerxiao" };
    who(p1);
    who(p2);
}

trait Action {
    fn run(&self);
    fn say(&self);
}

struct Person<'a> {
    name: &'a str,
}

impl Action for Person<'_> {
    fn run(&self) {
        println!("{} runing...", self.name);
    }
    fn say(&self) {
        println!("{} saying...", self.name);
    }
}

fn who(person: Person) {
    person.run();
    person.say();
}
