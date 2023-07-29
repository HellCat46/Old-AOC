use std::ops::Index;


fn main() {
    let data = std::fs::read_to_string("data/rucksacks.txt").unwrap();

    let mut tot_priorities : u32 = 0;

    for rucksack in data.split("\n") {
        let compartments = rucksack.split_at(rucksack.len()/2);
        for item in compartments.0.chars() {
            if compartments.1.contains(item){
                match item as u8 {
                    65..=90 => tot_priorities += (item as u32 - 65) + 27,
                    97..=122 => tot_priorities += item as u32 - 96,
                    _ => panic!("Something wrong with input")
                }
                break;
            }
        }
    }
    println!("Part 1: {}", tot_priorities);   

    // Part 2

    tot_priorities = 0;
    let rucksacks = data.split("\n").collect::<Vec<&str>>();

    let mut i = 0;
    while i < rucksacks.len() {
        
        let item = common_char(rucksacks.index(i), rucksacks.index(i+1), rucksacks.index(i+2));

        if let Some(item) = item {
            match item as u8 {
                65..=90 => tot_priorities += (item as u32 - 65) + 27,
                97..=122 => tot_priorities += item as u32 - 96,
                _ => panic!("Something wrong with input")
            }
        }
        i += 3;
    }
    println!("Part 2: {}", tot_priorities)

}


fn common_char(first : &str, second : &str, third : &str) -> Option<char>{

    let mut common : Option<char> = None;
    for chtr in first.chars() {
        if second.contains(chtr) && third.contains(chtr) {
           common = Some(chtr);
           break;
        }
    }
    common
}
