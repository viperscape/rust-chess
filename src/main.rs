
#[derive(Show,Copy)]
enum Item {
    Pawn,
    King,
    Queen,
    Rook,
    Knight, 
    Bishop,
}

type Position = (usize,usize); //change to u8 when rust gets changed!
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
}

#[derive(Show,Copy)]
enum Player {
    Black(Item),
    White(Item),
}

impl Player {
    /// check play logic for valid moves
    fn play_isvalid (&self, from: Position , to: Position, capturing: bool) -> bool {
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
}


type BoardLayout = [[Option<Player>;8];8];

#[derive(Show)]
struct Game {
    board: BoardLayout,
    captured: Vec<Player>,
    active: Player, //use a fake piece to set who is current active player
}

impl Game {
    fn new() -> Game  {
        let mut board = [[None;8];8];

        //setup pawns row
        board[1] = [Some(Player::White(Item::Pawn));8];
        board[6] = [Some(Player::Black(Item::Pawn));8];

        //setup home row
        board[0] = [Some(Player::White(Item::Rook)),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Queen)),
                    Some(Player::White(Item::King)),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Rook))];

        board[7] = [Some(Player::Black(Item::Rook)),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Queen)),
                    Some(Player::Black(Item::King)),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Rook))];
        Game { board:board, captured:Vec::new(), active: Player::White(Item::Pawn) }
    }

    fn get_player (&self,at:Position) -> &Option<Player> {
        &self.board[at.0][at.1]
    }

    /// swap out destination, and return it
    fn swap_pos (&mut self,at:Position, p:Option<Player>) -> Option<Player> {
        let oldp;
        if let &Some(_p) = self.get_player(at) {
            oldp = Some(_p);
        }
        else {oldp=None;}
        self.board[at.0][at.1] = p;
        oldp
    }

    fn play(&mut self, from:Position,to:Position) -> bool {
        println!("{:?}",self.get_player(from));
        println!("{:?}",self.get_player(to));

        if let &Some(p) = self.get_player(from) { 
            match (p,self.active) {
                (Player::White(_),Player::Black(_)) | 
                    (Player::Black(_),Player::White(_)) => return false,
                _ => (),
            }

            let capturing = self.capturing(from,to);
            if p.play_isvalid(from,to, capturing) {
                if let Some(_p) = self.swap_pos(to,Some(p)) {
                    println!("captured{:?}",_p);
                    self.captured.push(_p);
                }
                self.swap_pos(from,None);
                true
            }
            else {false}
        }
        else {false}
    }

    fn capturing (&self, from: Position, to: Position) -> bool {
        if let &Some(p) = self.get_player(to) {
            let res = match (p,self.get_player(from).unwrap()) {
                (Player::Black(_),Player::White(_)) => true,
                (Player::White(_),Player::Black(_)) => true,
                _ => false,
            };
            return res
        }
        false
    }
}

fn main() {
    let mut game = Game::new();
    println!("valid move? {}",game.play((7,0),(0,0)));
    println!("{:?}",game);
}
