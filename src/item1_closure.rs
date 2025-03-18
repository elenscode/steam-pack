// Item.1 - Closure
use num_traits::Num;

fn apply_lambda<T, F>(data: &mut [T], mutator: F)
where
    T: Num + Copy,
    F: Fn(T) -> T,
{
    for item in data {
        *item = mutator(*item);
    }
}

fn main() {
    let prefix_value = 30;
    let mut data = [30, 50, 70];
    apply_lambda(&mut data, |x| prefix_value + x);
    println!("{:?}", data);
}
