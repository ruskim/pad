use colored::Colorize;

use rand::seq::SliceRandom;
use rand::thread_rng;

struct Piece {
    cells: Vec<Vec<i8>>,
    bitmask: i64,
}

struct PieceClass {
    pieces: Vec<Piece>,
    id: i8,
}

struct PiecePosition {
    x: i8,
    y: i8,
    class: i8,
    form: i8,
}

struct PieceSet {
    classes: Vec<PieceClass>,
}

fn set_bit(bitmask: i64, x: i8, y: i8) -> i64{
    let pos = x + y*7;
    return  bitmask | (1<<pos); 
}

fn vec_to_bitmask(v: &Vec<Vec<i8>>) -> i64 {
        let sx = v[0].len();
        let sy = v.len();

        let mut bitmask: i64 = 0;
        for i in 0..sy {
            for j in  0..sx {
                if v[i][j] != 0 {
                    bitmask = set_bit(bitmask, j as i8, i as i8);
                }
            }
        }

        return bitmask;
}

fn build_piece(c: Vec<Vec<i8>>) -> Piece {
    let bitmask = vec_to_bitmask(&c);

    Piece {
        cells: c,
        bitmask: bitmask,
    }
}

fn has_piece( ps: &Vec<Piece>, p: &Piece) -> bool {
    for i in ps {
        if i.equal(p) {
            return true;
        }
    }
    false
}

fn build_piece_class(p: Piece, id: i8) ->PieceClass {
        let mut pieces: Vec<Piece> =  Vec::new();

        let p1 = p.rotate();
        let p2 = p1.rotate();
        let p3 = p2.rotate();

        let pf = p.flip();
        let pf1 = pf.rotate();
        let pf2 = pf1.rotate();
        let pf3 = pf2.rotate();

        let mut forms = vec![p, p1, p2, p3, pf, pf1, pf2, pf3];

        while forms.len() != 0 {
            let pp = forms.pop().unwrap();
            if !has_piece(&pieces, &pp) {
                pieces.push(pp)
            }
        }

        let mut rng = thread_rng();
        pieces.shuffle(&mut rng);

        PieceClass {
            pieces: pieces,
            id: id,
        }

}

impl PieceClass {
    #[allow(dead_code)]
    fn print(&self) {
        println!("PieceClass: {}", self.id);
        for r in &self.pieces {
            r.print();
        }
    }
}

impl Piece {
    #[allow(dead_code)]
    fn print(&self) {
        for r in &self.cells {
            for c in r {
                print!("{}", c);
            }
            println!("");
        }
        println!("");
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

        for i in 0..cells.len() {
            cells[i].reverse();
        }

        build_piece(cells)
    }

    fn flip(&self) -> Piece {
        let mut cells: Vec<Vec<i8>> =  Vec::new();

        for i in (0..self.cells.len()).rev() {
            cells.push(self.cells[i].clone());
        }
        build_piece(cells)
    }

    fn equal(&self, p: &Piece) -> bool {
        if self.cells.len() != p.cells.len() {
            return false;
        }

        if self.cells[0].len() != p.cells[0].len() {
            return false;
        }

        for i in 0..self.cells.len() {
            for j in 0..self.cells[0].len() {
                if self.cells[i][j] != p.cells[i][j] {
                    return false;
                }
            }
        }
        return true;
    }

    fn to_bitmask(&self, x: i8, y: i8) -> i64 {

        let sx = self.cells[0].len() as i8;
        let sy = self.cells.len() as i8;

        if sx + x > 7 {
            return -1;
        }

        if sy  + y > 7 {
            return -1;
        }

        return self.bitmask << (7*y + x);
    }

}

fn build_piece_set() -> PieceSet {
    let mut classes: Vec<PieceClass> =  Vec::new();

    let mut p = build_piece(
        vec![
            vec![1,1],
            vec![1,0],
            vec![1,0],
            vec![1,0]
            ]);
    classes.push(build_piece_class(p, 2));

    p = build_piece(
        vec![
            vec![1,0],
            vec![1,1],
            vec![1,0],
            vec![1,0]
            ]);
    classes.push(build_piece_class(p, 1));
    
    p = build_piece(
        vec![
            vec![1,1,0],
            vec![0,1,0],
            vec![0,1,1],
            ]);
    classes.push(build_piece_class(p, 4));
    
    p = build_piece(
        vec![
            vec![1,1],
            vec![1,0],
            vec![1,1],
            ]);
    classes.push(build_piece_class(p, 3));

    p = build_piece(
        vec![
            vec![1,1],
            vec![1,1],
            vec![1,1],
            ]);
    classes.push(build_piece_class(p, 5));
    
    p = build_piece(
        vec![
            vec![1,0,0],
            vec![1,0,0],
            vec![1,1,1],
            ]);
    classes.push(build_piece_class(p, 6));
    
    p = build_piece(
        vec![
            vec![1,0],
            vec![1,1],
            vec![0,1],
            vec![0,1]
            ]);
    classes.push(build_piece_class(p, 7));
    
    p = build_piece(
        vec![
            vec![1,0],
            vec![1,1],
            vec![1,1],
            ]);
    classes.push(build_piece_class(p, 8));

    let mut rng = thread_rng();
    classes.shuffle(&mut rng);

    PieceSet {
        classes: classes,
    }

}

impl PieceSet {
    #[allow(dead_code)]
    fn print(&self) {
        for pc in &self.classes {
            pc.print();
        }
    }

    fn iterate(&self, class: i8, state: i64, finish: i64, solution: &mut Vec<PiecePosition>) -> bool {
        if (class as usize) >= self.classes.len() {
            return false;
        }

        for (i, piece) in self.classes[class as usize].pieces.iter().enumerate() {
            for x in 0..7 {
                for y in 0..7 {
                    let mask = piece.to_bitmask(x, y);
                    if mask < 0 {
                        continue;
                    }
                    if mask & state != 0 {
                        continue;
                    }
                    let new_state = mask | state;
                    if new_state == finish {
//                        println!("Solution found!");
//                        println!("at {}:{}", x, y);
//                        piece.print();
                        solution.push(
                            PiecePosition{
                                x: x, 
                                y: y, 
                                class: class,
                                form: i as i8,
                            });
                        return true;
                    }
                    if self.iterate(class+1, new_state, finish, solution) {
//                        println!("at {}:{}", x, y);
//                        piece.print();
                        solution.push(
                            PiecePosition{
                                x: x, 
                                y: y, 
                                class: class,
                                form: i as i8,
                            });
                        return true;
                    }
                }
            }
        }

        return false;

    }

    fn solve(&self, state: i64, finish: i64) {
        let mut solution: Vec<PiecePosition> = Vec::new();

        if self.iterate(0, state, finish, &mut solution) {

            let blocks = [
                "\u{2588}".black(),
                "\u{2588}".blue(),
                "\u{2588}".red(),
                "\u{2588}".yellow(),
                "\u{2588}".green(),
                "\u{2588}".magenta(),
                "\u{2588}".cyan(),
                "\u{2588}".white(),
                "\u{2588}".bright_black(),
                "\u{2588}".white(),
                ];

            let months = [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", 
                "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

            let mut array_2d = [[0; 7]; 7];
            
            for p in &solution {
                let cls = &self.classes[p.class as usize];
                let f = &cls.pieces[p.form as usize];
                for (i, v) in f.cells.iter().enumerate() {
                    for (j, c) in v.iter().enumerate() {
                        if *c != 0 {
                            array_2d[i+(p.y as usize)][j+(p.x as usize)] = cls.id;
                        }
                    }
                }
//                println!("position: c:{}, p:{}, x:{}, y:{}", p.class, p.form, p.x, p.y);
            }

            println!("");
            for i in 0..array_2d.len() {
                for j in 0..array_2d[i].len() {
                    let c = array_2d[i][j];
                    if c == 0 {
                        if (i == 0 || i == 1) && j < 6 {
                            print!("{}", months[6*i+j as usize]);
                            continue;
                        }
                        if ((i >= 2) && (i <= 5)) || ((i == 6) && (j <= 2)) {
                            let d = (i-2)*7 + j + 1;
                            if d < 10 {
                                print!("{:>2} ", d);

                            } else {
                                print!("{:>3}", d);
                            }
                            continue;
                        }
                        //let p = "\u{25CD}";
                        //print!(" {} ", p);
                        //continue;
                    }
                    let b = &blocks[c as usize];
                    print!("{}{}{}", b, b, b);
                }
                println!("");
            }
            println!("");
        } else {
            println!("not found");
        }

    }
}

fn initial_state() -> i64 {
    let mut mask: i64 = 0;
    mask = set_bit(mask, 6, 0);
    mask = set_bit(mask, 6, 1);

    mask = set_bit(mask, 3,6);
    mask = set_bit(mask, 4,6);
    mask = set_bit(mask, 5,6);
    mask = set_bit(mask, 6,6);

    //23 Feb
    //mask = set_bit(mask, 1,0);
    //mask = set_bit(mask, 1,5);
    
    //mask = set_bit(mask, 3,0);
    //mask = set_bit(mask, 6,4);
    
    //mask = set_bit(mask, 0,0);
    //mask = set_bit(mask, 0,2);

    //6 Oct
    mask = set_bit(mask, 3, 1);
    mask = set_bit(mask, 5, 2);

    return mask;
}

fn final_state() -> i64 {
    return i64::pow(2, 49) - 1;
}


fn main() {
    let ps = build_piece_set();

//    ps.print();

    let state = initial_state();
    let finish = final_state();
//    println!("from {:b} to {:b}", state, finish);

    ps.solve(state,finish)
}
