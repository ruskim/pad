// use std::vec;

struct Piece {
    cells: Vec<Vec<i8>>,
}

fn build_piece(c: Vec<Vec<i8>>) -> Piece {
    Piece {
        cells: c,
    }
}

impl Piece {
    fn print_piece(&mut self) {
        for r in &self.cells {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
        self.cells[0].push(1);
    }
}

fn main() {
    let mut p = build_piece(vec![vec![0,1], vec![0,1]]);
    p.print_piece();
    p.print_piece();
}
