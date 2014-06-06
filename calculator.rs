// This code is editable and runnable!
fn main() {
    // A simple integer calculator:
    // `+` or `-` means add/sub by 1
    // `*` or `/` means mul/div by 2

    let program = "+ + * - /";
    let mut accumulator = 0;

    for token in program.chars() {
    match token { /* Includes spaces, too */
        '+' => accumulator += 1,
   	    '-' => accumulator -= 1,
        '*' => accumulator *= 2,
        '/' => accumulator /= 2,
        _ => { /* ignore everything else */ }
        }
	println!("accumulator is {}", accumulator);
    }

    println!("The program \"{}\" calculates the value {}",
              program, accumulator);
}
