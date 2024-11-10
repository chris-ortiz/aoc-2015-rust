pub fn q10() {
    let iterations = 50;

    let mut input: String = String::from("1113222113");
    for _ in 0..iterations {
        // println!("input {:?}", input);
        input = look_and_say(input);
        // println!("output {:?}", input);
    }

    println!("size: {}", input.len())
}

fn look_and_say(input: String) -> String {
    let input: Vec<char> = input.chars().collect();
    let mut result: Vec<char> = Vec::new();

    let mut index = 0;

    while index < input.len() {
        let mut count: usize = 1;
        let mut count_index: usize = index;

        while count_index < input.len() - 1 && input[count_index] == input[count_index + 1] {
            count += 1;
            count_index += 1
        }
        result.push(char::from_digit(count as u32, 10).unwrap());
        result.push(input.get(index).unwrap().clone());


        index += count
    }

    result.iter().collect()
}