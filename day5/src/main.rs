fn main() {
    let data = std::fs::read_to_string("data/ship.txt").unwrap();
    let (guide, mut ship) = parser(data);


    for order in guide {
        println!("{order:?}");
        for _ in 0..order[0] {
            let scrate = ship[(order[1] as usize) - 1].last().unwrap().to_string();
            ship[(order[1] as usize) -1].pop();
            
            ship[(order[2] as usize) - 1].push(scrate);
        }
        
    }


    print!("Crates on top of stacks: ");
    for rows in ship {
        print!("{} ", rows.last().unwrap());
    }
    print!("\n");


}

fn parser(data : String) -> (Vec<[i16;3]>, [Vec<String>; 9]) {
    let (pre_ship, instructions) = data.split_at(data.find("\n\n").unwrap());
    let mut guide : Vec<[i16;3]> = Vec::new();
    let mut ship : Vec<[String;9]> = Vec::new();

    println!("{pre_ship}");

    for instruction in instructions.split("\n") {
        if instruction.is_empty() {continue;}

        let mut t : [i16;3] = [0;3];
        let mut count = 0;
        for part in instruction.split(" "){
            if part.parse::<i16>().is_ok() {
                t[count] = part.parse().unwrap();
                count += 1;
            }
        }
        guide.push(t);
    }

    for line in pre_ship.split("\n") {

        const S : String = String::new();
        let mut t : [String;9] = [S; 9];

        for (count, character) in line.chars().enumerate() {
            if (count+1)%4 != 0{

                t[count/4] += character.to_string().as_str();
               
            }
        }
        ship.push(t);
    }

    (guide, transpose(ship))
}

fn transpose(ship : Vec<[String;9]>) -> [Vec<String>; 9]{
    const T : Vec<String> = Vec::new();
    let mut tras_ship : [Vec<String>; 9] = [T; 9];

    for row in ship.into_iter().rev() {
        
        for (count, ele) in row.iter().enumerate(){
            if ele == "   " { continue;}

            tras_ship[count].push(ele.to_owned());
            
        }
    }

    println!("\n\n\n\n{:?}", tras_ship);

    tras_ship
}