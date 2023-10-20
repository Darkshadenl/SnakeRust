enum Owner {
    Empty,
    Snake,
    Wall,
    Food,
}

impl Owner {
    fn as_char(&self) -> char {
        match *self {
            Owner::Empty => ' ',
            Owner::Snake => 'S',
            Owner::Wall => '#',
            Owner::Food => '*',
        }
    }
}

struct Snake {
    cells: Vec<Cell>,
}

impl Snake {
    fn new() -> Snake {
        Snake{
            cells: Vec::new(),
        }
    }
}

struct Cell {
    x: u32,
    y: u32,
    owner: Owner,
    father: Option<Box<Cell>>,
    child: Option<Box<Cell>>
}

impl Cell {
    fn new(x: u32, y: u32, owner: Owner) -> Cell {
        Cell {
            x,
            y,
            owner,
            father: None,
            child: None,
        }
    }
}


struct Board {
    width: u32,
    height: u32,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    fn new(width: u32, height: u32) -> Board {
        let mut board = Board {
            width,
            height,
            cells: Vec::new(),
        };
        board.create_board();
        board
    }

    fn create_board(mut self) -> Self {
        self.cells = (0..self.width).map(|i| {
            (0..self.height).map(|j| {
                Cell::new(i, j, Owner::Empty)
            }).collect()
        }).collect();
        self
    }
}

struct Game {
    board: Board,
    snake: Snake,
    food: Cell,
}

impl Game {
    fn new() -> Game {
        Game {
            board: Board::new(10, 10),
            snake: Snake::new(),
            food: Cell::new(0, 0, Owner::Food),
        }
    }
}

fn main() {
    let mut game = Game::new();
    let board = game.board.create_board();
    game.board.create_board();
    println!("Hello, snake!");
}
