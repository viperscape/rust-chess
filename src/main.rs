
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
        let (r,c) = abs_pos(from,to);
        if r < 2 &&
            r < 2 {true}
        else {false}
    }

    fn bishop_logic (&self, from:Position, to:Position) -> bool {
        let (r,c) = abs_pos(from,to);
        if r == c {true}
        else {false}
    }

    fn pawn_logic (&self, from:Position, to:Position, inverted:bool) -> bool {
        let (r,c) = abs_pos(from,to);
        if !inverted {
            if to.0 - from.0 == 1 &&
                c == 1 {true}  // diagonally on capture
            else if to.0 - from.0 < 3 &&
                c == 0 {true} //forward, 1 or 2 spaces (first move)
            else {false}
        }
        else { //black is playing
            if from.0 - to.0 == 1 &&
                c == 1 {true}  // diagonally on capture
            else if from.0 - to.0 < 3 &&
                c == 0 {true} //forward, 1 or 2 spaces (first move)
            else {false}
        }
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

    fn play_isvalid (&self, from:Position, to:Position, inverted:bool) -> bool {
        match *self {
            Item::Pawn => self.pawn_logic(from,to,inverted),
            Item::King => self.king_logic(from,to),
            Item::Queen => { // queen is in essence rook, bishop, and king combined
                if self.rook_logic(from,to) ||
                    self.bishop_logic(from,to) ||
                    self.king_logic(from,to) {true}
                else {false}
            },
            Item::Rook =>  self.rook_logic(from,to),
            Item::Knight => self.knight_logic(from,to),
            Item::Bishop => self.bishop_logic(from,to),
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
    fn play_isvalid (&self, from: Position , to: Position) -> bool {
        match *self {
            Player::Black(item) => item.play_isvalid(from,to,true),
            Player::White(item) => item.play_isvalid(from,to,false),
        }
    }
}


type BoardLayout = [[Option<Player>;8];8];

#[derive(Show)]
struct Game {
    board: BoardLayout,
    captured: Vec<Player>,
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
        Game { board:board, captured:Vec::new() }
    }

    fn get_player (&self,at:Position) -> &Option<Player> {
        &self.board[at.0][at.1]
    }

    fn play(&mut self, from:Position,to:Position)-> bool {
        println!("{:?}",self.get_player(from));
        println!("{:?}",self.get_player(to));

        if let &Some(p) = self.get_player(from) { p.play_isvalid(from,to) }
        else {false}
    }
}

fn main() {
    let mut game = Game::new();
   // println!("{:?}",game);
    println!("{}",game.play((0,0),(3,0)));
}
