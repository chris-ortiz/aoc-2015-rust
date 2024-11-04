pub fn q4() {
    let key = "yzbqklnj";


    for i in 0.. {
        let input = format!("{}{}", key, i);
        let digest = md5::compute(input);
        let hex = format!("{:x}", digest);

        if hex.starts_with("00000") {
            println!("Starting with five zeroes at {}", i);
        }

        if hex.starts_with("000000") {
            println!("Starting with six zeroes at {}", i);
            break;
        }
    }
}

