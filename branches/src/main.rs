fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("result of loop {}", result);

    let collection = ["a","b","c","d","e"];
    let mut index = 0;

    while index < 5 {
        println!("{}", collection[index]);
        index += 1;
    }

    for elem in collection.iter() {
        println!("{}", elem);
    }
}
