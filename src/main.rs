use rand::Rng;

#[derive(PartialEq)]
enum Movement {
    Up,
    Down,
    Left,
    Right
}

enum EntityType {
    Wall,
    Food
}

struct Entity {
    entity_type: EntityType,
    x: u32,
    y: u32
}

#[derive(Copy, Clone)]
struct SnakePart {
    x: u32,
    y: u32
}

struct Game {
    is_running: bool,
    is_alive: bool,
    width: u32,
    height: u32,
    movement: Option<Movement>,
    entities: Vec<Entity>,
    snake: std::collections::VecDeque<SnakePart>
}

impl Game {
    fn new() -> Self {
        let mut res = Self {
            is_running: true,
            is_alive:   true,
            width:      40,
            height:     20,
            movement:   None,
            entities:   Vec::new(),
            snake:      std::collections::VecDeque::new()
        };

        res.init_field();

        return res;
    }

    fn init_field(&mut self) {
        self.is_alive = true;
        self.movement = None;
        self.entities.clear();
        self.snake.clear();

        for x in 0..self.width {
            self.entities.push(Entity{
                entity_type: EntityType::Wall,
                x: x,
                y: 0
            });
            self.entities.push(Entity{
                entity_type: EntityType::Wall,
                x: x,
                y: self.height - 1
            });
        }
        for y in 1..self.height-1 {
            self.entities.push(Entity{
                entity_type: EntityType::Wall,
                x: 0,
                y: y
            });
            self.entities.push(Entity{
                entity_type: EntityType::Wall,
                x: self.width - 1,
                y: y
            });
        }

        let mut rng = rand::thread_rng();
        self.entities.push(Entity{
            entity_type: EntityType::Food,
            x: rng.gen_range(2..self.width-2),
            y: rng.gen_range(2..self.height-2)
        });

        self.snake.push_back(SnakePart{
            x: self.width / 2,
            y: self.height / 2
        });
    }

    fn step_forward(&mut self) {
        let head = *self.snake.back().unwrap();
        let tail = self.snake.pop_front().unwrap();

        match self.movement {
            Some(Movement::Up)    => self.snake.push_back(SnakePart{x: head.x, y: head.y - 1}),
            Some(Movement::Down)  => self.snake.push_back(SnakePart{x: head.x, y: head.y + 1}),
            Some(Movement::Left)  => self.snake.push_back(SnakePart{x: head.x - 1, y: head.y}),
            Some(Movement::Right) => self.snake.push_back(SnakePart{x: head.x + 1, y: head.y}),
            None => self.snake.push_front(tail)
        }
    }

    pub fn render(&self) {
        let mut field: Vec<Vec<char>> = Vec::with_capacity(self.height as usize);
        for _ in 0..self.height {
            field.push(vec!['.'; self.width as usize]);
        }

        for entity in self.entities.iter() {
            let rendered: char;

            match entity.entity_type {
                EntityType::Wall => rendered = '#',
                EntityType::Food => rendered = '@'
            }

            field[entity.y as usize][entity.x as usize] = rendered;
        }

        for snake_part in self.snake.iter().take(self.snake.len() - 1) {
            field[snake_part.y as usize][snake_part.x as usize] = '0';
        }

        let head = self.snake.back().unwrap();
        let head_char: char;
        match self.movement {
            Some(Movement::Up)    => head_char = '^',
            Some(Movement::Down)  => head_char = 'v',
            Some(Movement::Left)  => head_char = '<',
            Some(Movement::Right) => head_char = '>',
            None                  => head_char = '0'
        };
        field[head.y as usize][head.x as usize] = head_char;

        let mut buffer = String::new();
        for row in field.iter() {
            for c in row.iter() {
                buffer.push(*c);
            }
            buffer += "\n";
        }
        ncurses::addstr(&buffer);
        ncurses::addstr("\nScore: ");
        ncurses::addstr(&(self.snake.len()-1).to_string());
        ncurses::addstr("\n");

        if !self.is_alive {
            ncurses::addstr("\nYou died. Press 'R' to restart\n");
        }
    }

    pub fn update(&mut self) {
        match ncurses::getch() as u8 {
            27 => self.is_running = false,
            b'r' | b'R' => if !self.is_alive {
                self.init_field();
            }
            b'w' | b'W' => if self.movement != Some(Movement::Down) {
                self.movement = Some(Movement::Up)
            },
            b'a' | b'A' => if self.movement != Some(Movement::Right) {
                self.movement = Some(Movement::Left)
            },
            b's' | b'S' => if self.movement != Some(Movement::Up) {
                self.movement = Some(Movement::Down)
            },
            b'd' | b'D' => if self.movement != Some(Movement::Left) {
                self.movement = Some(Movement::Right)
            },
            _ => {}
        }

        if !self.is_alive {
            return;
        }

        self.step_forward();

        let mut rng = rand::thread_rng();
        let mut grow = false;

        let head = *self.snake.back().unwrap();
        for entity in self.entities.iter_mut() {
            if entity.x == head.x && entity.y == head.y {
                match entity.entity_type {
                    EntityType::Food => {
                        entity.x = rng.gen_range(2..self.width-2);
                        entity.y = rng.gen_range(2..self.height-2);
                        grow = true;
                    },
                    EntityType::Wall => {
                        self.is_alive = false;
                        return;
                    }
                }
            }
        }
        for part in self.snake.iter().take(self.snake.len() - 1) {
            if part.x == head.x && part.y == head.y {
                self.is_alive = false;
                return;
            }
        }

        if grow {
            self.snake.push_front(SnakePart{
                x: 0,
                y: 0
            });
            self.step_forward();
        }
    }
}

fn main() {
    let mut game = Game::new();

    let win = ncurses::initscr();
    ncurses::nodelay(win, true);

    while game.is_running {
        ncurses::clear();

        game.update();
        game.render();

        ncurses::refresh();

        std::thread::sleep(std::time::Duration::from_millis(200));
    }

    ncurses::endwin();
}
