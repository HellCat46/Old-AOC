

fn main() {
    let data = std::fs::read_to_string("data/cleaning_bros.txt").unwrap();


    let mut count = 0;
    for pair in data.split("\n") {

        let elfs : Vec<_>= pair.split(",").collect::<Vec<&str>>().iter().map(|i| i.split("-")).collect();
        
        let mut a : Vec<(u32,u32)> = Vec::new();
        for mut elf in elfs {
                a.push((elf.nth(0).unwrap().trim().parse().unwrap(), elf.nth(0).unwrap().trim().parse().unwrap()));
        }

        let cmp_first = a[0].0  >= a[1].0 && a[0].1 <= a[1].1;
        let cmp_sec = a[1].0  >= a[0].0 && a[1].1 <= a[0].1;
  
        if cmp_first || cmp_sec {
            count +=1;
        }
    }

    println!("{count}")
}
