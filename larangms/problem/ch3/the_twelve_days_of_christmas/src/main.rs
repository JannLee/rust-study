fn main() {    
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let refrain = [
        "And a partridge in a pear tree\n",
        "Two turtle doves",
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

    let mut lyrics : Vec<String> = Vec::new();

    for i in 0..12 {
        lyrics.push(format!("On the {} day of Christmas, my true love sent to me", days[i]));

        if i == 0 {
            lyrics.push("A partridge in a pear tree\n".to_string());
        }
        else {
            for j in (0..i + 1).rev() {
                lyrics.push(refrain[j].to_string());
            }
        }       
    }

    let str_lyrics = lyrics.join("\n");

    println!("{:#?}", lyrics);
    println!("\n\n");
    println!("{}",str_lyrics);

}
