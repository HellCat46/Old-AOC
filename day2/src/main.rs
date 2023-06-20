struct RPS {
    score : i32
}

fn main() {
    let data = std::fs::read_to_string("data/rps.txt").expect("Can't find file");
    let rounds = data.split("\n");

    let mut me = RPS { score : 0 };

    for round in rounds {
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

    println!("Score: {}", me.score);
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