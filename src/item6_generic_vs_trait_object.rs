fn print_value<T: std::fmt::Display>(value: T) {
    println!("Value: {}", value);
}

trait Animal {
    fn speak(&self);
}

struct Dog;
impl Animal for Dog {
    fn speak(&self) {
        println!("멍멍!");
    }
}

struct Cat;
impl Animal for Cat {
    fn speak(&self) {
        println!("야옹!");
    }
}

// Trait Object Type (&dyn Animal)
fn make_sound(animal: &dyn Animal) {
    animal.speak();
}

// Generic (T는 Animal 트레이트를 구현한 타입이어야 함)
fn make_sound<T: Animal>(animal: &T) {
    animal.speak();
}
