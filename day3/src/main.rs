

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

    println!("{}", tot_priorities);    
}