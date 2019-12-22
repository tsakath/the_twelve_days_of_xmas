fn main() {
    for n in 1..13 {
        println!("\n[Verse {}]", n);
        println!("On the {} day of xmas my true love sent to me", day(n));
        for k in (1..(n+1)).rev() {
            if k == 2 {
                println!("{}, and", gift(k));
                continue
            }
            println!("{} ", gift(k));
        }
    }
}

fn day(n: i32) -> &'static str {
    match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    }
}


fn gift(n: i32) -> &'static str {
    match n {
        1 => "A Partridge in a pear Tree",
        2 => "Two turtle doves",
        3 => "Three French hens",
        4 => "Four calling birds",
        5 => "Five golden rings",
        6 => "Six geese a laying",
        7 => "Seven swans a-swimming",
        8 => "Eight maids a-milking",
        9 => "Nine ladies dancing",
        10 => "10 lords a-leaping",
        11 => "11 pipers piping",
        12 => "12 drummers drumming",
        _ => "",
    }
}
