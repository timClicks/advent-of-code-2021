use std::str::FromStr;
struct Submarine {
    depth: i32,
    x: i32,
}

impl Submarine {
    fn step(&mut self, where_to: Movement) {
        match where_to {
            Movement::Forward(how_far) => self.forward(how_far),
            Movement::Backward(how_far) => self.backward(how_far),
            Movement::Up(how_far) => self.up(how_far),
            Movement::Down(how_far) => self.down(how_far),
        }
    }

    fn forward(&mut self, how_far: i32) {
        self.x += how_far;
    }
    
    fn backward(&mut self, how_far: i32) {
        self.x -= how_far;
    }

    fn up(&mut self, how_far: i32) {
        self.depth -= how_far;
    }

    fn down(&mut self, how_far: i32) {
        self.depth += how_far;
    }

    fn position(&self) -> i32 {
        self.depth * self.x
    }
}

enum Movement {
    Forward(i32),
    Backward(i32),
    Up(i32),
    Down(i32),
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split_whitespace();

        if let (Some(direction), Some(magnitude)) = (parts.next(), parts.next()) {        
            let mag = i32::from_str(magnitude).unwrap();

            let mov = match direction {
                "forward" => Movement::Forward(mag),
                "backward" => Movement::Backward(mag),
                "up" => Movement::Up(mag),
                "down" => Movement::Down(mag),
                _ => return Err(()),
            };
            return Ok(mov);
    
        }

        return Err(());
    }
}

fn main() {
    let input: Vec<_> = include_str!("input.txt")
        .lines()
        .map(|line| line.trim())
        .map(|line| line.parse::<Movement>() )
        .collect();

    let mut sub  = Submarine { depth: 0, x: 0 };
    for step in input {
        sub.step(step.unwrap());
    }

    println!("{}", sub.position());
}
