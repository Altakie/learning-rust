fn main() {
    let num_to_word = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "evelenth", "twelveth",
    ];

    let items = [
        "partridge in a pair tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "gold rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    let mut items_sent = String::new();

    for i in 0..(num_to_word.len()) {
        println!(
            "On the {} day of christmas my true love sent to me: ",
            num_to_word[i]
        );

        if i == 0 {
            items_sent = "a partridge in a pear tree".to_owned();
        } else {
            items_sent = (i + 1).to_string().to_owned() + &" " + &items[i] + &"\n" + &items_sent;
        }

        println!("{}", items_sent);

        if i == 0 {
            items_sent = "And ".to_owned() + &items_sent;
        }
    }
}
