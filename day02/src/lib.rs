pub struct Position {
    depth: u32,
    horizontal: u32,
    aim: u32,
}

impl Position {

    pub fn new(depth: u32, horizontal: u32, aim: u32) -> Position {
        Position {depth, horizontal, aim}
    }

    pub fn up(&mut self, v: u32) {
        self.aim -= v;
    }

    pub fn down(&mut self, v: u32) {
        self.aim += v;
    }

    pub fn forward(&mut self, v:u32) {
        self.horizontal += v;
        self.depth += self.aim * v
    }

    pub fn horizontal(&self) -> u32 {
        self.horizontal
    }

    pub fn depth(&self) -> u32 {
        self.depth
    }
    
}