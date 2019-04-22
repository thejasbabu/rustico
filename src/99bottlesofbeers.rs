// Problem statement: http://www.rosettacode.org/wiki/99_Bottles_of_Beer

fn main() {
    println!("--99 Bottles of Beer Program--");
    for bottles in (1..100_i32).rev() {
        print_lyrics(bottles);
    }
}

fn print_lyrics(bottles :i32 ) {
    println!("{} of beer on the wall", no_of_bottles(bottles));
    println!("{} of beer", no_of_bottles(bottles));
    println!("Take one down, pass it around");
    println!("{} of beer on the wall", no_of_bottles(bottles - 1));
}

fn no_of_bottles(bottles :i32) -> String {
    match bottles {
        0 => format!("No bottle"),
        1 => format!("1 bottle"),
        x => format!("{} bottles", x),
    }
}