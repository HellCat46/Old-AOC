use std::fs;

struct Result {
    elves_count : i32,
    highest_calories : i32,
    elf_with_highest : i32,
    all : Vec<Elf>
}
struct Elf {
    position : i32,
    calories : i32
}

fn main() {

    let data = get_data("../data/calories.txt");

    let elves_calories = total_calories(data);

    let result= find_result(elves_calories);


    println!("An Elf has {} calories worth of food at {} location.", result.highest_calories, result.elf_with_highest);
    println!("Total Elves: {} ", result.elves_count);

    println!("TOP 3 :");
    println!("Rank 1 : {} at {} location", result.all[0].calories, result.all[0].position);
    println!("Rank 1 : {} at {} location", result.all[1].calories, result.all[1].position);
    println!("Rank 1 : {} at {} location", result.all[2].calories, result.all[2].position);

    let total = result.all[0].calories + result.all[1].calories + result.all[2].calories;
    println!("Total Calories of all top Elves {}", total );

}

fn get_data(path : &str) -> String {

    let data = fs::read_to_string(path).expect("Something went wrong!");
    data
}

fn total_calories(data : String) -> Vec<Elf> {

    let elves = data.split(10u8  as char);
    let mut count  = 0;
    let mut calories : Vec<Elf> = (0..((((elves.clone()).count())+1) as i32)).map(|_x|Elf {
        calories : 0,
        position : 0
    }).collect();

    for elf in elves {
        if !(elf.as_bytes()).is_empty() {
            calories[count].calories  += elf.trim().parse::<i32>().expect("*Cries*");
            calories[count].position = count as i32;

        }
        else {
            count += 1;
        }
    }
    calories
}

fn find_result(calories : Vec<Elf>) -> Result {
    let mut calories = calories;
    calories.sort_by(|elf, elf2| elf2.calories.cmp(&elf.calories));

    Result {
        elves_count : calories.len() as i32,
        highest_calories : calories[0].calories,
        elf_with_highest : calories[0].position,
        all : calories
    }
}