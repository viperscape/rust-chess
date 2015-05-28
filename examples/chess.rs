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

fn main () {
    let mut game = Game::new();
    let mut win_dim = (1024,768);
    
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Chess".to_string(),
            Size { width: win_dim.0, height: win_dim.1 }
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

    let mut menu_state = Menu::Main;
    
    for event in event_iter {
        ui.handle_event(&event);

        if let Some(args) = event.render_args() {
            gl.draw(args.viewport(), |c, gl| {
                let mut offset = PaneId+1;
                
                // Draw the background.
                Background::new().rgb(0.2, 0.2, 0.2).draw(ui, gl); //this swaps buffers for us
                Split::new(PaneId).color(dark_grey()).set(ui);

                build_menu_ui(&mut offset,ui, &mut game, &mut menu_state);
                build_board_ui(&mut offset,ui, &game, &win_dim, &menu_state);
                
                
                // Draw our Ui!
                ui.draw(c,gl);

            });
        }
    }
}

fn build_board_ui (offset: &mut usize, ui: &mut Ui<GlyphCache>, game: &Game, win_dim: &(u32,u32), menu_state: &Menu) {
    match *menu_state {
        Menu::Game => {
            let item_dim = (win_dim.0 as f64/8.2, win_dim.1 as f64/10.0);

            for (i,r) in game.view().iter().enumerate() {
                *offset += 8;
                for (j,c) in r.iter().enumerate() {
                    let mut b: Button<_>;
                    if (i == 0) & (j == 0) {
                        b = Button::new().bottom_left_of(PaneId);
                    }
                    else if j == 0 {
                        let mut id = *offset-8;
                        b = Button::new().up_from(UiId::Widget(id-j),5.0);
                    }
                    else {
                        b = Button::new().right(5.0);
                    }

                    b.label("X")
                        .dimensions(item_dim.0, item_dim.1)
                        .react(|| {
                        })
                        .set(*offset+j, ui);
                }
            }

            *offset += 8;
        },
        _=>(),
    }
}

fn build_menu_ui (offset: &mut usize, ui: &mut Ui<GlyphCache>, game: &mut Game, menu_state: &mut Menu) {
    match *menu_state {
        Menu::Main => {
            *offset +=1;
            Button::new()
                .top_left_of(PaneId)
                .label("New Game")
                .dimensions(200.0, 60.0)
                .react(|| {
                    *menu_state = Menu::Game;
                    *game = Game::new();
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
                    *menu_state = Menu::Main;
                })
                .set(*offset, ui);
        },
        //_ => (),
    }
}
