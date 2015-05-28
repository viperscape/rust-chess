extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

extern crate chess;
use chess::{Game,Player,Item,AN};


use conrod::{Background, Colorable, Theme, Ui, UiId, Positionable, Widget,WidgetId, Button, Labelable,Sizeable};
use conrod::color::{blue, light_grey, orange, dark_grey, red, white};
use conrod::{Label, Split, WidgetMatrix, Floating};

use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;

const PaneId:usize = 0;

/// menu states
enum Menu {
    Main,
    Game,
}

/// game state for game ui
struct GameState {
    menu: Menu,
    select: (Option<chess::Position>,Option<chess::Position>),
    game: Game,
    win_dim: (u32,u32),
}

impl GameState {
    fn default () -> GameState {
        GameState {
            menu: Menu::Main,
            select: (None,None),
            game: Game::new(),
            win_dim: (1024,768),
        }
    }
}

fn main () {
    let mut gs = GameState::default();
    
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Chess".to_string(),
            Size { width: gs.win_dim.0, height: gs.win_dim.1 }
            )
            .exit_on_esc(true)
            .samples(4)
            );


    let event_iter = window.events().ups(180).max_fps(60);
    let mut gl = GlGraphics::new(opengl);
    let font_path = Path::new("fonts/SourceCodePro-Regular.otf");
    let theme = Theme::default();
    let glyph_cache = GlyphCache::new(&font_path).unwrap();
    let mut ui = &mut Ui::new(glyph_cache, theme);
    
    for event in event_iter {
        ui.handle_event(&event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                let mut offset = PaneId+1;
                
                // Draw the background.
                Background::new().rgb(0.2, 0.2, 0.2).draw(ui, gl); //this swaps buffers for us
                Split::new(PaneId).color(dark_grey()).set(ui);

                build_menu_ui(&mut offset,ui, &mut gs);
                build_board_ui(&mut offset,ui, &mut gs);

                if let Some(from) = gs.select.0 {
                    if let Some(to) = gs.select.1 {
                        println!("play:{:?}",gs.game.play(from,to));
                        gs.select = (None,None);
                    }
                }
                
                // Draw our Ui!
                ui.draw(c,gl);

            });
        }
    }
}

fn build_board_ui (offset: &mut usize, ui: &mut Ui<GlyphCache>, gs: &mut GameState) {
    match gs.menu {
        Menu::Game => {
            let item_dim = (gs.win_dim.0 as f64/8.2, gs.win_dim.1 as f64/10.0);

            for (i,r) in gs.game.view().iter().enumerate() {
                *offset += 8;
                for (j,piece) in r.iter().enumerate() {
                    let mut b: Button<_>;
                    if (i == 0) & (j == 0) {
                        b = Button::new().bottom_left_of(PaneId);
                    }
                    else if j == 0 {
                        let id = *offset-8;
                        b = Button::new().up_from(UiId::Widget(id-j),5.0);
                    }
                    else {
                        b = Button::new().right(5.0);
                    }

                    // todo: convert to fmt for player pieces
                    let mut label = "";
                    if let Some(player) = *piece {
                        match player {
                            Player::Black(item) => {
                                label = match item {
                                    Item::Pawn => "Pawn",
                                    Item::Rook(_) => "Rook",
                                    Item::Knight => "Knight",
                                    Item::Bishop => "Bishop",
                                    Item::King(_) => "King",
                                    Item::Queen => "Queen",
                                    _ => "",
                                };
                            },
                            Player::White(item) => {
                                label = match item {
                                    Item::Pawn => "Pawn",
                                    Item::Rook(_) => "Rook",
                                    Item::Knight => "Knight",
                                    Item::Bishop => "Bishop",
                                    Item::King(_) => "King",
                                    Item::Queen => "Queen",
                                    _ => "",
                                };
                            },
                        }
                    }

                    //if let Some(ref pos) = gs.select.0 {
                    //    if *pos == (i as i8,j as i8) { b.color(blue()); }
                    //}
                    
                    b.label(label)
                        .dimensions(item_dim.0, item_dim.1)
                        .react(|| {
                            if gs.select.0.is_some() {
                                if !gs.select.1.is_some() { gs.select.1 = Some((i as i8,j as i8)); }
                            }
                            else { gs.select.0 = Some((i as i8,j as i8)); }
                        })
                        .set(*offset+j, ui);
                }
            }

            *offset += 8;
        },
        _=>(),
    }
}

fn build_menu_ui (offset: &mut usize, ui: &mut Ui<GlyphCache>, gs: &mut GameState) {
    match gs.menu {
        Menu::Main => {
            *offset +=1;
            Button::new()
                .top_left_of(PaneId)
                .label("New Game")
                .dimensions(200.0, 60.0)
                .react(|| {
                    gs.menu = Menu::Game;
                    gs.game = Game::new();
                })
                .set(*offset, ui);
            
            *offset +=1;
            Button::new()
                .right(10.0)
                .label("Load Game")
                .dimensions(200.0, 60.0)
                .react(|| {
                })
                .set(*offset, ui);
        },
        Menu::Game => {
            *offset +=1;
            Button::new()
                .top_left_of(PaneId)
                .label("Exit Game")
                .dimensions(200.0, 60.0)
                .react(|| {
                    gs.menu = Menu::Main;
                })
                .set(*offset, ui);
        },
        //_ => (),
    }
}
