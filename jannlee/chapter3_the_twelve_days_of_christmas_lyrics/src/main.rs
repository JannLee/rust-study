fn main() {
    let ordinals = [
        "first",
        "second",
        "third",
        "fourth",
        "fifth",
        "sixth",
        "seventh",
        "eighth",
        "ninth",
        "tenth",
        "eleventh",
        "twelfth"];

    let presents = [
        "partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming"
    ];

    for (i, ordinal) in ordinals.iter().enumerate() {
        println!("On the {} day of Christmas", ordinal);
        println!("my true love gave to me:");
        for (j, present) in presents[..i + 1].iter().rev().enumerate() {
            if i == j {
                if i == 0 {
                    println!("A {}", present);
                }
                else {
                    println!("And a {}", present);
                }
            }
            else {
                println!("{}", present);
            }
        }
        println!("");
    }
}
