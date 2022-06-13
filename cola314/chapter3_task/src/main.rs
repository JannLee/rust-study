fn main() {
    print!("{}", lyrics_of_the_twelve_days_of_christmas());
}

#[allow(dead_code)]
fn to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.0
}

#[allow(dead_code)]
fn to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 1.8
}

#[allow(dead_code)]
fn fibonacci(n: u32) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn lyrics_of_the_twelve_days_of_christmas() -> String {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let items = [
        "Two turtle doves, and",
        "Three french hens",
        "Four calling birds",
        "Five golden rings",
        "Six geese a-laying",
        "Seven swans a-swimming",
        "Eight maids a-milking",
        "Nine ladies dancing",
        "Ten lords a-leaping",
        "Eleven pipers piping",
        "Twelve drummers drumming",
    ];

    let mut lyrics = String::new();
    for i in 0..12 {
        lyrics.push_str(
            format!(
                "On the {} day of Christmas, my true love sent to me\n",
                days[i]
            )
            .as_str(),
        );
        for j in (0..i).rev() {
            lyrics.push_str(format!("{}\n", items[j]).as_str());
        }
        lyrics.push_str("A partridge in a pear tree\n");
    }

    lyrics
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_fahrenheit() {
        assert_eq!(to_fahrenheit(10.0), 50.0);
    }

    #[test]
    fn test_to_celsius() {
        assert_eq!(to_celsius(50.0), 10.0);
    }

    #[test]
    fn test_fibonacci() {
        assert_eq!(fibonacci(12), 144);
    }

    #[test]
    fn test_lyrics_of_the_twelve_days_of_christmas() {
        assert_eq!(
            lyrics_of_the_twelve_days_of_christmas(),
            "On the first day of Christmas, my true love sent to me
A partridge in a pear tree
On the second day of Christmas, my true love sent to me
Two turtle doves, and
A partridge in a pear tree
On the third day of Christmas, my true love sent to me
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the fourth day of Christmas, my true love sent to me
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the fifth day of Christmas, my true love sent to me
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the sixth day of Christmas, my true love sent to me
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the seventh day of Christmas, my true love sent to me
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the eighth day of Christmas, my true love sent to me
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the ninth day of Christmas, my true love sent to me
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the tenth day of Christmas, my true love sent to me
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the eleventh day of Christmas, my true love sent to me
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
On the twelfth day of Christmas, my true love sent to me
Twelve drummers drumming
Eleven pipers piping
Ten lords a-leaping
Nine ladies dancing
Eight maids a-milking
Seven swans a-swimming
Six geese a-laying
Five golden rings
Four calling birds
Three french hens
Two turtle doves, and
A partridge in a pear tree
"
        );
    }
}
