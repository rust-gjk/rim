use std::io::{self, Read, Write};

fn main() {
	println!("zadejte směr převodu: římské -> arabské (1), arabské -> římské (2) (výchozí = 1). Následně zadávejte čísla, jedno na řádek");
	let mut buf = String::new();
	let mut stdin = io::stdin();

	stdin.read_line(&mut buf);
	let r2a = buf.trim().parse::<u8>().unwrap_or(1);
	buf.clear();

	'outer: while stdin.read_line({buf.clear(); &mut buf}).is_ok() {
		if r2a == 1 {
			let mut vysledek = 0;
			let mut chars = buf.trim().chars().peekable();

			loop {
				match chars.next() {
					Some('M') => vysledek+=1000,
					Some('L') => if "CDM".contains(&chars.peek().unwrap_or(&'Y').to_string()) { vysledek -= 50 } else { vysledek += 50 },
					Some('X') => if "LCDM".contains(&chars.peek().unwrap_or(&'Y').to_string()) { vysledek -= 10 } else { vysledek += 10 },
					Some('V') => vysledek += 5,
					Some('I') => if "VLCDM".contains(&chars.peek().unwrap_or(&'Y').to_string()) { vysledek -= 1 } else { vysledek += 1 },
					Some('C') => if "DM".contains(&chars.peek().unwrap_or(&'Y').to_string()) { vysledek -= 100 } else { vysledek += 100 },
					Some('D') => vysledek += 500,
					Some(_)   => { println!("špatný vstup"); continue 'outer }
					None => break,
				}
			}
			println!("výsledek: {}", vysledek);
		}
		else if buf == "konec" { break 'outer; }
		else {
			let mut cislo = match buf.trim().parse::<u32>() {
				Ok(c) => c,
				Err(_) => { println!("špatný vstup"); continue 'outer },
			};

			while cislo != 0 {
				if cislo >= 1000 { print!("M"); cislo -= 1000; }
				else if cislo >= 500 { print!("D"); cislo -= 500; }
				else if cislo >= 100 { print!("C"); cislo -= 100; }
				else if cislo >= 50  { print!("L"); cislo -= 50;  }
				else if cislo >= 10  { print!("X"); cislo -= 10;  }
				else if cislo == 9  { print!("IX"); cislo -= 9;  }
				else if cislo >= 5   { print!("V"); cislo -= 5;   }
				else if cislo == 4  { print!("IV"); cislo -= 4;  }
				else if cislo > 0   { print!("I"); cislo -= 1;   }
				else {break;}
			}

			io::stdout().flush().expect("e");
		}
	}
}
