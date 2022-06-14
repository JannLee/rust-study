fn main() {
    let day = [
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
        "11th",
        "12th"
    ];

    let present = [
        "partridge in a pear tree",
        "Two turtle-doves",
        "Three French hens",
        "Four calling birds",
        "Five golden rings (five golden rings)",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "12 drummers drumming"
    ];

    for number in 0..12 {
        println!("On the {} day of Christmas", day[number]);
        println!("My true love sent to me");
        let mut index = number;
        while 0 < index {
            println!("{}", present[index]);
            index=index-1;
        }
        if number == 0 {
            println!("A {}", present[index]);
        }
        else {
            println!("And {}", present[index]);
        }
        println!();
    }
}