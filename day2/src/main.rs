struct RPS {
    score : i32
}

fn main() {
    let data = std::fs::read_to_string("data/rps.txt").expect("Can't find file");
    let rounds = data.split("\n");
  
    let mut me = RPS { score : 0 };
    for round in rounds.clone() {
        match round {
            "A X" => me.draw(1),
            "A Y" => me.won(2),
            "A Z" => me.lost(3) ,
            "B X" => me.lost(1),
            "B Y" => me.draw(2),
            "B Z" => me.won(3),
            "C X" => me.won(1),
            "C Y" => me.lost(2),
            "C Z" => me.draw(3),
            _ => ()
        }
    }

    println!("Part 1: {}", me.score);

    

    let possibilities = [
        "B X", "C X", "A X",
        "A Y", "B Y", "C Y",
        "C Z", "A Z", "B Z"
    ];
    let mut points = 0;

    for round in rounds {
        
        let outcome = match  possibilities.iter().position(|&poss| poss == round) {
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
    println!("Part 2: {}", points);
}

impl RPS {
    fn won(&mut self, bonus : i32) {
        self.score += 6 + bonus;
    }
    fn draw(&mut self, bonus : i32) {
        self.score += 3 + bonus;
    }
    fn lost(&mut self, bonus : i32) {
        self.score += bonus;
    }
}