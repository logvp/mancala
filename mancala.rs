use std::io;

#[derive(Debug)]
enum Outcome {
    Winner(Player),
    Tie,
    NotOver,
}
impl Outcome {
    pub fn is_over(&self) -> bool {
        !matches!(self, Self::NotOver)
    }
}

#[derive(Debug)]
enum Player {
    White,
    Black,
}

enum MoveStatus {
    GoAgain,
    OutOfBounds,
    EmptyCell,
    Ok,
}

#[derive(Debug)]
struct Side {
    side: [u32; 6],
    points: u32,
}

#[derive(Debug)]
struct Board {
    white: Side,
    black: Side,
}
impl Board {
    const LENGTH: usize = 6;

    pub fn new(initial_fill: u32) -> Board {
        Board {
            white: Side {
                side: [initial_fill; Self::LENGTH],
                points: 0,
            },
            black: Side {
                side: [initial_fill; Self::LENGTH],
                points: 0,
            },
        }
    }

    pub fn white_move(&mut self, index: usize) -> MoveStatus {
        Self::action(&mut self.white, &mut self.black, index - 1)
    }

    pub fn black_move(&mut self, index: usize) -> MoveStatus {
        Self::action(&mut self.black, &mut self.white, index - 1)
    }

    fn action(this: &mut Side, other: &mut Side, mut index: usize) -> MoveStatus {
        let mut hand: u32 = match this.side.get(index) {
            None => return MoveStatus::OutOfBounds,
            Some(0) => return MoveStatus::EmptyCell,
            Some(&value) => value,
        };
        this.side[index] = 0;

        index += 1;
        loop {
            let (pickupable, cell): (bool, &mut u32) = if index < this.side.len() {
                (true, this.side.get_mut(index).unwrap())
            } else if index == this.side.len() {
                (false, &mut this.points)
            } else {
                let bindex = index - this.side.len();
                if bindex < other.side.len() {
                    (true, other.side.get_mut(bindex).unwrap())
                } else {
                    index = 0;
                    continue;
                }
            };

            match (pickupable, hand, *cell) {
                (_, 2.., _) => {
                    hand -= 1;
                    *cell += 1;
                }
                (true, 1, 1..) => {
                    hand += *cell;
                    *cell = 0;
                }
                (true, 1, 0) => {
                    *cell += 1;
                    return MoveStatus::Ok;
                }
                (false, 1, _) => {
                    *cell += 1;
                    return MoveStatus::GoAgain;
                }
                (_, 0, _) => unreachable!(),
            }

            index += 1;
        }
    }

    pub fn state(&self) -> Outcome {
        if self.white.side.iter().sum::<u32>() == 0 || self.black.side.iter().sum::<u32>() == 0 {
            if self.white.points > self.black.points {
                Outcome::Winner(Player::White)
            } else if self.black.points > self.white.points {
                Outcome::Winner(Player::Black)
            } else {
                Outcome::Tie
            }
        } else {
            Outcome::NotOver
        }
    }

    pub fn print(&self) {
        assert_eq!(Self::LENGTH, 6);
        println!(" White   (6)  (5)  (4)  (3)  (2)  (1)");
        let side = self.white.side;
        println!(
            "[{:>5}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}]",
            self.white.points, side[5], side[4], side[3], side[2], side[1], side[0]
        );
        let side = self.black.side;
        println!(
            "        [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:<5}]",
            side[0], side[1], side[2], side[3], side[4], side[5], self.black.points
        );
        println!("         (1)  (2)  (3)  (4)  (5)  (6)  Black");
    }
}

fn get_input(buffer: &mut String) -> io::Result<usize> {
    Ok(loop {
        buffer.clear();
        io::stdin().read_line(buffer)?;
        match buffer.trim().parse::<usize>() {
            Ok(value) => break value,
            Err(_) => println!("Invalid selection!"),
        }
    })
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut board = Board::new(4);

    'game_loop: loop {
        loop {
            board.print();
            if board.state().is_over() {
                break 'game_loop;
            }
            println!("White move:");
            match board.white_move(get_input(&mut buffer)?) {
                MoveStatus::Ok => break,
                MoveStatus::EmptyCell => println!("You can't select an empty cell!"),
                MoveStatus::OutOfBounds => println!("Out of bounds!"),
                MoveStatus::GoAgain => println!("Go again!"),
            }
        }
        loop {
            board.print();
            if board.state().is_over() {
                break 'game_loop;
            }
            println!("Black move:");
            match board.black_move(get_input(&mut buffer)?) {
                MoveStatus::Ok => break,
                MoveStatus::EmptyCell => println!("You can't select an empty cell!"),
                MoveStatus::OutOfBounds => println!("Out of bounds!"),
                MoveStatus::GoAgain => println!("Go again!"),
            }
        }
    }
    
    match board.state() {
        Outcome::Winner(winner) => println!("Winner: {:?}", winner),
        Outcome::Tie => println!("Tie Game!"),
        Outcome::NotOver => unreachable!(),
    }

    Ok(())
}

