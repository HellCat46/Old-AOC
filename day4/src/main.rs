

fn main() {
    let data = std::fs::read_to_string("data/cleaning_bros.txt").unwrap();


    let mut count = 0;
    let mut p2_count = 0;
    for pairs in data.split("\n") {

        let elfs : Vec<_>= pairs.split(",").collect::<Vec<&str>>().iter().map(|i| i.split("-")).collect();
        
        let mut pair : Vec<(u32,u32)> = Vec::new();
        for mut elf in elfs {
                pair.push((elf.nth(0).unwrap().trim().parse().unwrap(), elf.nth(0).unwrap().trim().parse().unwrap()));
        }

        let cmp_first = pair[0].0  >= pair[1].0 && pair[0].1 <= pair[1].1;
        let cmp_sec = pair[1].0  >= pair[0].0 && pair[1].1 <= pair[0].1;
        
        if cmp_first || cmp_sec {
            count +=1;
        }

        // Part 2

        if pair[0].0 <= pair[1].1 && pair[1].0 <= pair[0].1{
            p2_count += 1;
        }

        
    }

    println!("Part 1: {count} \nPart 2: {p2_count}")
}


