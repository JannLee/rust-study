fn main() {
    const DAY_COUNT : usize = 12;
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "11th", "12th"];
    let presents = ["a partridge in a pear tree", 
    "Two turtle-doves", 
    "Three French hens", 
    "Four calling birds",
    "Five golden rings (five golden rings)",
    "Six geese a laying",
    "Seven swans a swimming",
    "Eight maids a-milking", 
    "Nine ladies dancing", 
    "Ten lords a-leaping", 
    "I sent 11 pipers piping", 
    "12 drummers drumming",
    ];

    for i in 0..DAY_COUNT {
        println!("On the {} day of Christmas", days[i]);
        println!("My true love sent to me");
        
        for j in (0..=i).rev() {
            println!("{}", presents[j]);
        }
        println!("\n");
    }
}
