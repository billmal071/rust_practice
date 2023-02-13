pub fn num_as_roman(num: i32) -> String {
    let mut num = num;
    let mut result = String::new();

    let my_vec = vec![
        (1000, "M"),
        (900, "CM"),
        (500, "D"),
        (400, "CD"),
        (100, "C"),
        (90, "XC"),
        (50, "L"),
        (40, "XL"),
        (10, "X"),
        (9, "IX"),
        (5, "V"),
        (4, "IV"),
        (1, "I"),
    ];
    for (key, value) in my_vec.iter() {
        let times = num / key;
        println!("times: {}", times);
        for _ in 0..times {
            result.push_str(value);
        }
        num %= key;
    }

    result
}
