extern crate conrod;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate piston;

extern crate chess;
use chess::{Game,Player,Item,AN};


use conrod::{Background, Colorable, Theme, Ui, Positionable, Widget,WidgetId, Button, Labelable,Sizeable};
use conrod::color::{blue, light_grey, orange, dark_grey, red, white};
use conrod::{Label, Split, WidgetMatrix, Floating};

use glutin_window::GlutinWindow;
use opengl_graphics::{ GlGraphics, OpenGL };
use opengl_graphics::glyph_cache::GlyphCache;
use piston::event::*;
use piston::window::{ WindowSettings, Size };
use std::path::Path;

fn main () {
    let mut game = Game::new();

    
    let opengl = OpenGL::_3_2;
    let window = GlutinWindow::new(
        opengl,
        WindowSettings::new(
            "Chess".to_string(),
            Size { width: 1024, height: 768 }
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

                // Draw the background.
                Background::new().rgb(0.2, 0.2, 0.2).draw(ui, gl); //this swaps buffers for us

                build_board_ui(0,ui, &game);
                
                // Draw our Ui!
                ui.draw(c,gl);

            });
        }
    }
}

fn build_board_ui (offset: usize, ui: &mut Ui<GlyphCache>, game: &Game) {
    Split::new(offset+1).color(dark_grey()).set(ui);
    
    Button::new()
        .bottom_left_of(offset+1)
        .label("X")
        .dimensions(40.0, 40.0)
        .react(|| {
        })
        .set(offset+2, ui);

    let offset = offset+2;

    for (i,r) in game.view().iter().enumerate() {
        for (j,c) in r.iter().enumerate() {
            Button::new()
                .right(5.0)
                .label("X")
                .dimensions(40.0, 40.0)
                .react(|| {
                })
                .set(offset+j, ui);
        }
    }
}
