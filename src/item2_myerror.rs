//item 2 error
#[derive(Debug)]
enum MyError {
    Io(std::io::Error),
    General(String),
}

impl std::fmt::Display for MyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MyError::Io(err) => write!(f, "I/O error: {}", err),
            MyError::General(msg) => write!(f, "Error: {}", msg),
        }
    }
}

fn main() -> Result<(), MyError> {
    let file = std::fs::File::open("abc").map_err(MyError::Io)?;

    Ok(())
}
