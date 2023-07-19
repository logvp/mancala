use std::{cmp::Ordering, io, rc::Rc};

enum Outcome {
    Winner(Rc<str>),
    Tie,
    NotOver,
}
impl Outcome {
    pub fn is_over(&self) -> bool {
        !matches!(self, Self::NotOver)
    }
}

enum MoveStatus {
    GoAgain,
    OutOfBounds,
    EmptyCell,
    Done,
}

#[derive(Debug, Clone)]
struct Player {
    pub name: Rc<str>,
    side: Vec<u32>,
    points: u32,
}

#[derive(Debug)]
struct Board {
    whos_up: usize,
    players: Vec<Player>,
}
impl Board {
    const LENGTH: usize = 6;

    pub fn new(initial_fill: u32, player_names: &[&str]) -> Board {
        Board {
            whos_up: 0,
            players: player_names
                .iter()
                .map(|&name| Player {
                    name: Rc::from(name),
                    side: vec![initial_fill; Self::LENGTH],
                    points: 0,
                })
                .collect(),
        }
    }

    pub fn player(&self) -> Rc<str> {
        self.players[self.whos_up].name.clone()
    }

    pub fn turn(&mut self, mut index: usize) -> MoveStatus {
        let player = self.whos_up;
        let mut player_index = player;
        let mut hand: u32 = match self.players[player].side.get(index) {
            None => return MoveStatus::OutOfBounds,
            Some(0) => return MoveStatus::EmptyCell,
            Some(&value) => value,
        };
        self.players[player].side[index] = 0;

        index += 1;
        loop {
            let (pickupable, cell) = if index < self.players[player_index].side.len() {
                (
                    true,
                    self.players[player_index].side.get_mut(index).unwrap(),
                )
            } else if player_index == player && index == self.players[player].side.len() {
                (false, &mut self.players[player].points)
            } else {
                player_index = (player_index + 1) % self.players.len();
                index = 0;
                continue;
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
                    self.whos_up = (self.whos_up + 1) % self.players.len();
                    return MoveStatus::Done;
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
        if self.players.is_empty()
            || self
                .players
                .iter()
                .any(|player| player.side.iter().sum::<u32>() == 0)
        {
            match self
                .players
                .iter()
                .map(Option::Some)
                .reduce(|max, x| match (max, x) {
                    (None, _) => None,
                    (_, None) => unreachable!(),
                    (Some(a), Some(b)) => match Ord::cmp(&a.points, &b.points) {
                        Ordering::Less => x,
                        Ordering::Equal => None,
                        Ordering::Greater => max,
                    },
                })
                .flatten()
            {
                Some(player) => Outcome::Winner(player.name.clone()),
                None => Outcome::Tie,
            }
        } else {
            Outcome::NotOver
        }
    }

    pub fn print(&self) {
        assert_eq!(Self::LENGTH, 6);
        let width = self
            .players
            .iter()
            .map(|player| player.name.len())
            .max()
            .unwrap_or_default()
            + 2;
        let format_name = |name, pad| format!("{pad}{}{pad}", name);
        for i in 0..self.players.len() {
            let player = &self.players[i];
            let side = &player.side;
            let pad = if i == self.whos_up { '*' } else { ' ' };
            if i % 2 == 0 {
                println!(
                    " {:^width$}   (6)  (5)  (4)  (3)  (2)  (1)",
                    format_name(self.players[i].name.clone(), pad)
                );
                println!(
                    "[{:^width$}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}]",
                    player.points, side[5], side[4], side[3], side[2], side[1], side[0]
                );
            } else {
                println!(
                    "{:<width$}   [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:>2}] [{:^width$}]",
                    "",
                    side[0],
                    side[1],
                    side[2],
                    side[3],
                    side[4],
                    side[5],
                    self.players[i].points
                );
                println!(
                    " {: <width$}   (1)  (2)  (3)  (4)  (5)  (6)  {:^width$}",
                    "",
                    format_name(self.players[i].name.clone(), pad)
                );
            }
        }
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
    let mut board = Board::new(4, &["White", "Black"]);

    loop {
        board.print();
        if board.state().is_over() {
            break;
        }
        println!("{}'s move:", board.player());
        match board.turn(get_input(&mut buffer)? - 1) {
            MoveStatus::Done => (),
            MoveStatus::EmptyCell => println!("You can't select an empty cell!"),
            MoveStatus::OutOfBounds => println!("Out of bounds!"),
            MoveStatus::GoAgain => println!("Go again!"),
        }
    }

    match board.state() {
        Outcome::Winner(winner) => println!("Winner: {}", winner),
        Outcome::Tie => println!("Tie Game!"),
        Outcome::NotOver => unreachable!(),
    }

    Ok(())
}
