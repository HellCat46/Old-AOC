

fn main() {
    let data = std::fs::read_to_string("data/rps.txt").expect("WEE WOO");
    let rounds = data.split("\n");

    let possibilities = [
        "B X", "C X", "A X",
        "A Y", "B Y", "C Y",
        "C Z", "A Z", "B Z"
    ];
    let mut points = 0;

    for round in rounds.enumerate() {
        
        let outcome = match  possibilities.iter().position(|&poss| poss == round.1) {
            Some(postion) => postion,
            None => panic!("Something is wrong with input")
        };
        
        if outcome >= 3 && outcome <= 5{
            points += 3;
        }else if outcome >= 6 && outcome <= 8  {
            points += 6;
        }
        points += (outcome % 3) + 1;
    }
    println!("{}", points);
}