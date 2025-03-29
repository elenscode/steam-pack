#[derive(PartialOrd, PartialEq)]
struct Oddity(f32);

fn main() {
    let x = Oddity(f32::NAN);
    let y = Oddity(f32::NAN);
    if x <= y {
        println!("thie lins dosn't get excuted!!");
    }
}
