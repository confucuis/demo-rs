pub trait Animal {
    fn make_sound(&self);
}

pub struct Cat;
pub struct Dog;

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

pub fn sound_off(animal: &dyn Animal) {
    animal.make_sound();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
