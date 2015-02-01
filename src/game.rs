use super::{Player,Item,Position,MoveType};

pub type BoardLayout = [[Option<Player>;8];8];

#[derive(Debug)]
pub struct Game {
    board: BoardLayout,
    captured: Vec<Player>,
    active: Player, //use a fake piece to set who is current active player
}

impl Game {
    pub fn new() -> Game  {
        let mut board = [[None;8];8];

        //setup pawns row
        board[1] = [Some(Player::White(Item::Pawn));8];
        board[6] = [Some(Player::Black(Item::Pawn));8];

        //setup home row
        board[0] = [Some(Player::White(Item::Rook(false))),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Queen)),
                    Some(Player::White(Item::King(false))),
                    Some(Player::White(Item::Bishop)),
                    Some(Player::White(Item::Knight)),
                    Some(Player::White(Item::Rook(false)))];

        board[7] = [Some(Player::Black(Item::Rook(false))),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Queen)),
                    Some(Player::Black(Item::King(false))),
                    Some(Player::Black(Item::Bishop)),
                    Some(Player::Black(Item::Knight)),
                    Some(Player::Black(Item::Rook(false)))];
        Game { board:board, captured:Vec::new(), active: Player::White(Item::Pawn) }
    }

    pub fn get_player (&self,at:Position) -> &Option<Player> {
        &self.board[at.0][at.1]
    }

    /// swap out destination, and return original
    fn swap_pos (&mut self,at:Position, p:Option<Player>) -> Option<Player> {
        let oldp;
        if let &Some(_p) = self.get_player(at) {
            oldp = Some(_p);
        }
        else {oldp=None;}
        self.board[at.0][at.1] = p;
        oldp
    }

    pub fn play (&mut self, from:Position,to:Position) -> PlayResult {
        println!("{:?}",self.get_player(from));
        println!("{:?}",self.get_player(to));

        if let &Some(player) = self.get_player(from) { //must select an actual piece

            //current active player is playing?
            match (player,self.active) {
                (Player::White(_),Player::Black(_)) | 
                (Player::Black(_),Player::White(_)) => return PlayResult::Illegal,
                _ => (),
            }
            
            //only capturing other player's pieces, or nothing at all?
            if let &Some(oppo) =  self.get_player(to) {
                match (player, oppo) {
                    (Player::White(_),Player::White(_)) | 
                    (Player::Black(_),Player::Black(_)) => return PlayResult::Illegal,
                    _ => (),
                }
            }


            if let Some(mt) = player.play_isvalid(from,to, self.capturing(from,to)) {
                let path = player.play_path(from,to).pop(); //get path and remove dest
                let res = path.iter().find(|&n| self.get_player(*n).is_some());
                if res.is_some() { //blocked
                    return PlayResult::Blocked(*res.unwrap()) 
                }

                let _cap: Option<Player>;
                if let Some(_p) = self.swap_pos(to,Some(player)) { //captured
                    _cap = Some(_p);
                    self.captured.push(_p);
                }
                else { _cap = None; }
                
                // match the movetypes
                match mt {
                    MoveType::Castle => {
                        let _item = self.swap_pos(to,Some(player));
                        self.swap_pos(from,_item);
                    },
                    MoveType::Double(pos) => { 
                        //swap in the enpass ghost
                        match (player) {
                            Player::White(_) => {self.swap_pos(pos,Some(Player::White(Item::EnPass(to))));},
                            Player::Black(_) => {self.swap_pos(pos,Some(Player::White(Item::EnPass(to))));},
                        }
                        self.swap_pos(from,None);
                    },
                    MoveType::Upgrade => {
                        match (player) {
                            Player::White(_) => {self.swap_pos(to,Some(Player::White(Item::Queen)));},
                            Player::Black(_) => {self.swap_pos(to,Some(Player::Black(Item::Queen)));},
                        }
                        self.swap_pos(from,None);
                    },
                    MoveType::Regular => {self.swap_pos(from,None);}, //clear the space it came from
                }

                return PlayResult::Ok(_cap);
            }
        }
        
        PlayResult::Invalid
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

//todo: consider combining invalid and illegal?
#[derive(Debug)]
pub enum PlayResult {
    Ok(Option<Player>),
    Blocked(Position),
    Invalid, //not a valid move, according to logic
    Illegal, //a move that is valid, but not legal
}
