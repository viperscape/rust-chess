#[derive(Show,Copy)]
pub enum Item {
    Pawn,
    King,
    Queen,
    Rook,
    Knight, 
    Bishop,
}

pub type Position = (usize,usize); //change to u8 when rust gets changed!
fn abs (v: usize) -> usize { //wtf happened to std abs? also change to u8 soon
    let v = v as i32;
    if v < 0i32 {
        (v * -1) as usize
    } 
    else {v as usize}
}
fn abs_pos (from:Position,to:Position) -> (usize,usize) {
    let r = abs(to.0 - from.0);
    let c = abs(to.1 - from.1);
    (r,c)
}

impl Item {
    fn rook_logic (&self, from:Position, to:Position) -> bool {
        if to.0 == from.0 ||
            to.1 == from.1 {true}
        else {false}
    }

    fn king_logic (&self, from:Position, to:Position) -> bool {
        let (r,_) = abs_pos(from,to);
        if r < 2 &&
            r < 2 {true}
        else {false}
    }

    fn bishop_logic (&self, from:Position, to:Position) -> bool {
        let (r,c) = abs_pos(from,to);
        if r == c {true}
        else {false}
    }

    /// returns valid move, plus if it is a capturing move
    fn pawn_logic (&self, from:Position, to:Position, inverted:bool) -> (bool,bool) {
        let (_,c) = abs_pos(from,to);
        if !inverted {
            if to.0 - from.0 == 1 &&
                c == 1 {(true,true)}  // diagonally on capture
            else if to.0 - from.0 < 3 &&
                c == 0 {(true,false)} //forward, 1 or 2 spaces (first move)
            else {(false,false)}
        }
        else { //black is playing
            if from.0 - to.0 == 1 &&
                c == 1 {(true,true)}  // diagonally on capture
            else if from.0 - to.0 < 3 &&
                c == 0 {(true,false)} //forward, 1 or 2 spaces (first move)
            else {(false,false)}
        }
    }

    //todo: rename fn & also consider 'en passant' move within here?
    //note: en passant requires previous moves, or boardlayout
    fn pawn_islegal (&self, res: (bool,bool), capturing: bool) -> bool {
        if res.0 { //partially valid move? now determine if capture is valid
            if res.1 && capturing {true}
            else if !res.1 && !capturing {true} //nothing in way?
            //else if !res.1 && capturing {false} //not a diagonal move, and blocked
            else {false} //should cover any other case
        }
        else {false}
    }

    fn knight_logic (&self, from:Position, to:Position) -> bool {
        let (r,c) = abs_pos(from,to);

        if r < 1 || c < 1 ||
            r > 2 || c > 2 {false}
        else { 
            if r == 2 && c == 1 {true}
            else if r == 1 && c == 2 {true}
            else {false}
        }
    }

    //todo: workout if through to destination is valid (no blocking pieces!)
    fn play_isvalid (&self, from:Position, to:Position) -> bool {
        if from == to {return false}
        match *self {
            Item::King => self.king_logic(from,to), //todo: check for incidental checks when moving king, illegal
            Item::Queen => { // queen is in essence rook, bishop, and king combined
                if self.rook_logic(from,to) ||
                    self.bishop_logic(from,to) ||
                    self.king_logic(from,to) {true}
                else {false}
            },
            Item::Rook =>  self.rook_logic(from,to),
            Item::Knight => self.knight_logic(from,to),
            Item::Bishop => self.bishop_logic(from,to),
            _ => false,
        }
    }

    fn queen_path (&self, from:Position, to:Position) -> Vec<Position> {
        if self.rook_logic(from,to) { self.rook_path(from,to) }
        else if self.bishop_logic(from,to) { self.bishop_path(from,to) }
        else { vec!(to) }
    }

    fn rook_path (&self, from:Position, to:Position) -> Vec<Position> {
        let mut v = Vec::new();

        // heading down row or column?
        if from.0 != to.0 {
            for n in range(from.0,to.0) { v.push((n,from.1)) }
        }
        else {
            for n in range(from.1,to.1) { v.push((from.0,n)) }
        }

        v
    }

    fn bishop_path (&self, from:Position, to:Position) -> Vec<Position> {
        let mut v = Vec::new();
        let mut m = from.1;

        for n in range(from.0,to.0) {
            if from.1 > to.1 { m-=1; }
            else { m+=1; }
            v.push((n,m));
        }

        v
    }

    /// gets play path, to be checked later for if legal
    fn play_path (&self, from:Position, to:Position) -> Vec<Position> {
        match *self {
            Item::Queen => self.queen_path(from,to),
            Item::Rook =>  self.rook_path(from,to),
            Item::Bishop => self.bishop_path(from,to),
            _ => vec!(to), //single space destination
        }
    }
}

#[derive(Show,Copy)]
pub enum Player {
    Black(Item),
    White(Item),
}

impl Player {
    /// check play logic for valid moves
    pub fn play_isvalid (&self, from: Position , to: Position, capturing: bool) -> bool {
        match *self {
            Player::Black(item) => {
                match item {
                    Item::Pawn => { //pawn logic is special
                        let res = item.pawn_logic(from,to,true);
                        item.pawn_islegal(res,capturing)
                    },
                    _ => item.play_isvalid(from,to),
                }
            }
            Player::White(item) => {
                match item {
                    Item::Pawn => {
                        let res = item.pawn_logic(from,to,false);
                        item.pawn_islegal(res,capturing)
                    },
                    _ => item.play_isvalid(from,to),
                }
            }
        }
    }

    pub fn play_path (&self, from: Position , to: Position) -> Vec<Position> {
        match *self {
            Player::Black(item) => item.play_path(from,to),
            Player::White(item) => item.play_path(from,to),
        }
    }
}
