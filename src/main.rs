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
    fn print(&self) {
        for r in &self.cells {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
    }
    
    fn rotate(&self) -> Piece {
        let mut i = 0;
        let mut cells: Vec<Vec<i8>> =  Vec::new();
        for r in &self.cells {
            let mut j = 0;
            for c in r {
                if i == 0 {
                    cells.push(Vec::new())
                }
                cells[j].push(*c);
                j += 1;
            }
            i += 1;
        };

        i = 0;
        while i<cells.len() {
            cells[i].reverse();
            i += 1;
        }

        build_piece(cells)
    }
}

fn main() {
    let p = build_piece(vec![vec![0,1], vec![0,1]]);
    let p2 = p.rotate();
    let p3 = p2.rotate();
    let p4 = p3.rotate();
    p.print();
    p2.print();
    p3.print();
    p4.print();
}
