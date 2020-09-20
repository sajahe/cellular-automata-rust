extern crate sdl2;
extern crate bit_array;
extern crate image;
extern crate typenum;

use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
extern crate rand;

use std::env;
use std::thread;
use std::time::Duration;
mod elementary;
mod game_of_life;
mod setup;
use self::rand::prelude::*;

const CELL_SIZE: u32 = 2;
const HELP: &str = "cellular_automata COMMAND";

/**
* These are the console command rules that should be implented
* modes
*  graphics: uses the sdl to view conways game of life
*       Options in form B012345678S012345678
*  elementary
*       Rule: in form 0-255
*
*
*/
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut query = &String::from("");
    let mut rule = &String::from("");
    

    //Here the rule should be get. If no rule than some default rule.
    match args.len() {
        1 => {
            println!("No second argument!");

        },
        2 => {
            query = &args[1];
            
        },
        3=> {
            query = &args[1];
            rule = &args[2];
        },

        _ => println!("Unknown COMMAND {}\n Help: {}", query, HELP),
    }

    println!("Using {} {}", query, rule);

    match query.as_str() {
        "rule" => create_image(&rule),
        "graphics" => create_game_of_life_graphics(),
        _ => println!("Unknown COMMAND {}\n Help: {}", query, HELP),
    }
}


fn create_random_generation_vector(gen: &mut Vec<game_of_life::CellCoordinates>, amount: u32) {
    gen.clear();
    for _ in 0..amount {
        gen.push(game_of_life::build_cell_coordinates(
            rand::thread_rng().gen_range(200, 290) as u32,
            rand::thread_rng().gen_range(190, 280) as u32,
            true,
        ));
    }
}

fn create_image(rule: &String) {
    match rule.parse::<u8>() {
        Ok(n) => elementary::create_image(n),
        Err(e) => println!("Not a number {}", e),
    }
}

fn create_game_of_life_graphics() {
    println!("Game of life");
    let life_rule = setup::build_life_rule(
        [false, false, false, true, false, false, true, false, false],
        [false, true, true, false, false, true, false, false, false],
    );
    let mut first_gen: Vec<game_of_life::CellCoordinates> = Vec::new();
    create_random_generation_vector(&mut first_gen, 500);

    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let _window = video_subsystem
        .window("Game of Life", 900, 700)
        .resizable()
        .build()
        .unwrap();
    let mut canvas = _window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 0, 0));

    canvas.clear();
    for cell in &first_gen {
        if cell.alive {
            canvas.set_draw_color(Color::RGB(0, 255, 0));
        } else {
            canvas.set_draw_color(Color::RGB(30, 30, 30));
        }
        canvas
            .fill_rect(Rect::new(
                2 * cell.x as i32,
                2 * cell.y as i32,
                CELL_SIZE,
                CELL_SIZE,
            ))
            .unwrap();
    }

    let mut generation = game_of_life::create_next_generation_concurrent(&first_gen, &life_rule);

    canvas.present();

    let mut event_pump = _sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                sdl2::event::Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    println!("Reset");
                    create_random_generation_vector(&mut generation, 500)
                }
                _ => {}
            }
        }
        thread::sleep(Duration::new(0, 1_000_000_000u32 / 20));
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        for cell in &generation {
            if cell.alive {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
            } else {
                canvas.set_draw_color(Color::RGB(30, 30, 30));
            }
            canvas
                .fill_rect(Rect::new(
                    2 * cell.x as i32,
                    2 * cell.y as i32,
                    CELL_SIZE,
                    CELL_SIZE,
                ))
                .unwrap();
        }

        canvas.present();
        generation = game_of_life::create_next_generation_concurrent(&generation, &life_rule);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let mut first_gen: Vec<game_of_life::CellCoordinates> = Vec::new();
        create_random_generation_vector(&mut first_gen, 300);
        assert_eq!(300, first_gen.len());
    }
}
