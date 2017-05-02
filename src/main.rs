#[macro_use]
extern crate clap;
extern crate rust_cli_starter;

fn main() {
    let matches = clap_app!(app =>
    (name: "github CLI")
    (version: "0.1.0")
    (author: "kkpoon <noopkk@gmail.com>")
    (about: "Just for fun")
    (@subcommand users =>
      (about: "get a github user")
      (@arg USERNAME: +required "github username")
    )
  )
            .get_matches();

    if let Some(matches) = matches.subcommand_matches("users") {
        let username = matches.value_of("USERNAME").unwrap();
        let gh_user = rust_cli_starter::github::get_user(username);

        match gh_user {
            Ok(user) => println!("{}, {}", user.name, user.html_url),
            Err(err) => println!("Error: {}", err),
        }
    }

}
