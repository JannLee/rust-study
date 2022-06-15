
fn main() {
    let ordinal_numbers = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "11th", "12th"];
    let un_patterned_lyrics = [
        "12 drummers drumming",
        "11 pipers piping",
        "10 lords a-leaping",
        "9 ladies dancing",
        "8 maids a-milking",
        "7 swans a-swimming",
        "6 geese a-laying",
        "5 golden rings",
        "4 calling birds",
        "3 french hens",
        "2 turtle doves, and"
    ];

    for i in 0..12 {
        println!("On the {} day of Christmas", ordinal_numbers[i]);
        println!("My true love sent to me");
        
        for j in 11-i..11 {
            println!("{}", un_patterned_lyrics[j]);
        }
    
        println!("A partridge in a pear tree\n");
    }
    println!("A partridge in a pear tree");
}