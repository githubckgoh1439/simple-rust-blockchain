

// use std::error::Error;
// use std::fmt;

// #[derive(Fail, Debug)]
// #[fail(display = "There is an error: {}.", _0)]
// struct Sampleerror(String);

// impl fmt::Display for Sampleerror {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "There is an error: {}", self.0)
//     }
// }

// impl Error for Sampleerror {}

// pub fn run() -> Result<(), Box<dyn Error>> {
//     let condition = true;

//     if condition {
//         return Err(Box::new(MyError("Oops".into())));
//     }

//     Ok(())
// }

// fn main() {
//     if let Err(e) = run() {
//         println!("{}", e); // "There is an error: Oops"
//     }
// }



