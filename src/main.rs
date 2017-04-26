#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(app =>
        (name: "Rust CLI starter")
        (version: "0.1.0")
        (author: "kkpoon <noopkk@gmail.com>")
        (about: "command line tool starter")
        (@arg INPUT: +required "some text")
        (@subcommand upper =>
            (about: "convert to upper case")
        )
        (@subcommand lower =>
            (about: "convert to lower case")
        )
    )
            .get_matches();

    let input_text = matches.value_of("INPUT").unwrap();

    if let Some(_) = matches.subcommand_matches("upper") {
        println!("{}", input_text.to_uppercase());
    } else if let Some(_) = matches.subcommand_matches("lower") {
        println!("{}", input_text.to_lowercase());
    }
}
