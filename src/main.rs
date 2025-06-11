use std::collections::VecDeque;

mod moves;

type Disc = u8;

#[derive(Debug)]
struct Tower {
    peg: VecDeque<Disc>,
}

impl Tower {
    fn init(discs: &[Disc]) -> Self {
        let mut peg = VecDeque::new();
        for disc in discs {
            peg.push_front(*disc);
        }
        Self { peg } 
    }

    fn take_disk(&mut self, disk_id: u8) -> u8 {
        let taken = self.peg.pop_front().expect("Tower is empty");
        if taken != disk_id {
            panic!("Invalid move: Cannot take {disk_id}: Top disk is {taken}");
        }
        taken
    }

    fn put_disk(&mut self, disk_id: u8) {
        if self.peg.is_empty() {
            self.peg.push_front(disk_id);
            return;
        }
        if let Some(top_disk) = self.peg.pop_front() {
            if disk_id >= top_disk {
                panic!("Invalid move: Cannot put {disk_id} on top of {top_disk}");
            }
            self.peg.push_front(top_disk);
        }
        self.peg.push_front(disk_id);
    }
}

#[derive(Debug)]
struct Board {
    tower0: Tower,
    tower1: Tower,
    tower2: Tower,
}

impl Board {
    fn init(t0: &[Disc], t1: &[Disc], t2: &[Disc]) -> Self {
        Self {
            tower0: Tower::init(t0),
            tower1: Tower::init(t1),
            tower2: Tower::init(t2),
        }
    }

    fn apply_move(&mut self, move_index: usize, m: &[u8; 3]) {
        let [disk_id, from_peg, to_peg] = m;
        let move_number = move_index + 1;
        println!("Move number {move_number}: Moving disk {disk_id} from peg {from_peg} to peg {to_peg}");

        let disk = match from_peg {
            0 => self.tower0.take_disk(*disk_id),
            1 => self.tower1.take_disk(*disk_id),
            2 => self.tower2.take_disk(*disk_id),
            _ => panic!("Invalid `from_peg` index: {from_peg}"),
        };

        match to_peg {
            0 => self.tower0.put_disk(disk),
            1 => self.tower1.put_disk(disk),
            2 => self.tower2.put_disk(disk),
            _ => panic!("Invalid `to_peg` index: {to_peg}"),
        };
    }
}

fn main() {
    println!("The illusion of thinking.\n");

    let peg0 = &[10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
    let peg1 = &[];
    let peg2 = &[];
    let mut board = Board::init(peg0, peg1, peg2);
    println!("Starting configuration:\n{board:#?}\n");

    for (i, m) in moves::MOVES.iter().enumerate() {
        board.apply_move(i, m);
    }

    println!("\nResult:\n{board:#?}");
}
