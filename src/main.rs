//                                                       |@.......@.......|
//   main.rs                                             |................|
//                                                       |.ELF............|
//   By: 0xSpyC <rtazlaou@student.42mulhouse.fr>         |..>..... .......|
//                                                       |@........4......|
//   Created: 2024/01/05 17:05:11 by 0xSpyC              |....@.8...@.....|
//   Updated: 2024/01/07 19:13:30 by 0xSpyC              |........@.......|

use clap::{Arg, Command};
use polynomial::Polynomial;

fn main() {
    let matches = Command::new("computorv1") // requires `cargo` feature
        .version("0.1.0")
        .author("Reda Tazlaoui <rtazlaou@student.42mulhouse.fr>")
        .about("Polynomial Solver")
        .arg(
            Arg::new("polynomial")
                .required(true)
                .help("\"Ao * X^0 + .. + An * X^n = 0\""),
        )
        .get_matches();

    let equation: &str = matches.get_one::<String>("polynomial").unwrap().as_str();
    let polynome_parse = Polynomial::from_str(equation);
    match polynome_parse {
        Ok(mut polynome) => {
            polynome.reduce();
            polynome.display();
            match polynome.solve() {
                Ok(()) => {}
                Err(e) => eprintln!("{}", e.message()),
            }
        }
        Err(e) => eprintln!("{}", e.message()),
    }
}
