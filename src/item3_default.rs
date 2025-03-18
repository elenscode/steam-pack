#[derive(Debug, Default)]
struct Details {
    name: String,
    preferred_name: Option<String>,
    date_of_birth: std::time::Duration,
}

fn main() {
    let d = Details {
        name: "rk".to_owned(),
        ..Default::default()
    };
    println!("{:?}", d);
}
