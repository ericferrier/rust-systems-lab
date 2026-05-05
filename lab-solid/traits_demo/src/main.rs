trait Animal {
    fn speak(&self);
}

// --------------------
// Concrete types
// --------------------

struct Dog;

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow!");
    }
}

// --------------------
// Generic function (compile-time polymorphism)
// --------------------

fn make_sound<T: Animal>(animal: T) {
    animal.speak();
}

// --------------------
// Trait object function (runtime polymorphism)
// --------------------

fn make_sound_dynamic(animal: &dyn Animal) {
    animal.speak();
}

// --------------------
// Main
// --------------------

fn main() {
    let dog = Dog;
    let cat = Cat;

    // 1. Generic (static dispatch)
    make_sound(dog);
    make_sound(cat);

    // 2. Trait object (dynamic dispatch)
    make_sound_dynamic(&Dog);
    make_sound_dynamic(&Cat);
}