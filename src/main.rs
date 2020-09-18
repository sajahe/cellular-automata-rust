extern crate sdl2;

extern crate image;
extern crate bit_array;


extern crate typenum;

use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use std::collections::HashMap;
extern crate rand;

use typenum::U3;
use std::env;
use std::thread;
use std::time::Duration;
mod elementary;
mod game_of_life;
mod setup;
use self::rand::prelude::*;

// Construct a new ImageBuffer with the specified width and height.
//https://doc.rust-lang.org/book/ch11-01-writing-tests.html
fn create_random_generation(gen:&mut HashMap<game_of_life::Coordinates, bool>) {

    gen.clear();
    for _ in 0..300{
        //first_gen.push(game_of_life::build_cell_coordinates(rand::thread_rng().gen_range(200, 290) as u32,rand::thread_rng().gen_range(190, 280) as u32,true));
        gen.insert(game_of_life::Coordinates{x:rand::thread_rng().gen_range(200, 290) as u32,y:rand::thread_rng().gen_range(190, 280) as u32},true);
    }
    //return gen_formatted;

}

fn create_random_generation_vector(gen:&mut Vec<game_of_life::CellCoordinates>) {

    gen.clear();
    for _ in 0..300{
        //first_gen.push(game_of_life::build_cell_coordinates(rand::thread_rng().gen_range(200, 290) as u32,rand::thread_rng().gen_range(190, 280) as u32,true));
        gen.push(game_of_life::build_cell_coordinates(rand::thread_rng().gen_range(200, 290) as u32,rand::thread_rng().gen_range(190, 280) as u32,true));
    }
    //return gen_formatted;

}
//
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
    //let mut matrix2: [[bool; 16];8];
    //Here the rule should be get. If no rule than some default rule.
    let args: Vec<String> = env::args().collect();
    let cell_size = 2;
    let life_rule = setup::build_life_rule([false,false,false,true,false,false,true,false,false], [false,true,true,false,false,true,false,false,false]);
    //let life_rule = setup::build_life_rule([false,false,true,true,false,false,false,false,false], [false,false,false,true,true,false,false,false,false]);
    //let life_rule = setup::build_life_rule([false,false,true,false,true,false,false,false,false], [false,false,false,false,true,false,false,false,false]);
    //let life_rule = setup::build_life_rule([false,false,true,false,false,false,false,false,false], [false,false,false,false,false,false,false,false,false]);
    //let life_rulegraphics = ::setup::build_life_rule([false,false,false,true,false,false,false,false,false],
    //                                        [false,false,true,true,false,false,false,false,false]);
    //assert_eq!(true, check_rule(&[true,false,false],30));
    println!("{}",args[0]);
    match args[1].as_ref() {
        "elementary" => println!("Elementary"),
        "gol" => println!("Game of life also"),
        "game" => println!("Game of life"),
        _ => println!("Unknown COMMAND {}\n Usage: cellular_automata COMMAND",args[1])
    }

    let query = &args[1];

    let rule = &args[2];
    println!("Using {} {}", query, rule);
    if query == "rule"{
    match rule.parse::<u8>() {
        Ok(n) => elementary::create_image(n),
        Err(e) => println!("Not a number {}",e),
    }
    }
    let x=3;
    let y=4;
    let mut array: [bool; 32] = [false; 32];
    array[16] = true;
    //create_image();
    println!("{:?}", array);
    let rule = 30;
    let mut c = 0;

    if query == "graphics" {
        let mut first_gen: Vec<game_of_life::CellCoordinates> = Vec::new();
        //let mut first_gen: HashMap<game_of_life::Coordinates, bool> = HashMap::new();
        if rule == 24 {
        /*
        first_gen.insert(game_of_life::Coordinates{x:10,y:11},true);
        first_gen.insert(game_of_life::Coordinates{x:10,y:12},true);
        first_gen.insert(game_of_life::Coordinates{x:10,y:13},true);
        first_gen.insert(game_of_life::Coordinates{x:18,y:23},true);
        first_gen.insert(game_of_life::Coordinates{x:19,y:24},true);
        first_gen.insert(game_of_life::Coordinates{x:20,y:22},true);
        first_gen.insert(game_of_life::Coordinates{x:20,y:23},true);
        first_gen.insert(game_of_life::Coordinates{x:20,y:24},true);
        */
        } else {
            println!("Game of life");
            //create_random_generation(&mut first_gen);
            create_random_generation_vector(&mut first_gen);
            //return;
        }

        //let generation= game_of_life::create_next_generation(first_gen);
    let _sdl = sdl2::init().unwrap();
    let video_subsystem = _sdl.video().unwrap();
    let _window = video_subsystem
        .window("Game of Life",900,700)
        .resizable()
        .build()
        .unwrap();
        let mut canvas  =  _window.into_canvas().build().unwrap();

        canvas.set_draw_color(Color::RGB(0, 0, 0));

        canvas.clear();
        for cell in  &first_gen {
            println!("Game of life");
            if cell.alive {
                canvas.set_draw_color(Color::RGB(0, 255, 0));
            } else {
                canvas.set_draw_color(Color::RGB(30, 30,30));
            }
            canvas.fill_rect(Rect::new(2*cell.x as i32, 2*cell.y as i32, cell_size, cell_size));
        }
        /*
        for (key,value) in &first_gen {
            if *value {
                canvas.set_draw_color(Color::RGB(0, 255, 0));

            }else{
                canvas.set_draw_color(Color::RGB(30, 30,30));
            }
            canvas.fill_rect(Rect::new(2*key.x as i32, 2*key.y as i32, cell_size, cell_size));

        }
        */



        let mut generation= game_of_life::create_next_generation_concurrent(&first_gen, &life_rule);
        //let mut generation= game_of_life::create_next_generation_concurrent_hash(&first_gen, &life_rule);

        canvas.present();

        let mut event_pump = _sdl.event_pump().unwrap();
            'main: loop {

                for event in event_pump.poll_iter() {
                    match event {
                        sdl2::event::Event::Quit {..} => break 'main,
                        sdl2::event::Event::KeyDown { keycode: Some(Keycode::Return), .. } => {
                            println!("Reset");
                            create_random_generation_vector(&mut generation)
                            /*generation.clear();
                            for i in 0..300{

                                generation
.insert(game_of_life::Coordinates{x:rand::thread_rng().gen_range(200, 290) as u32,y:rand::thread_rng().gen_range(190, 280) as u32},true);
//.push(game_of_life::build_cell_coordinates(rand::thread_rng().gen_range(200, 290) as u32,rand::thread_rng().gen_range(190, 280) as u32,true));
}*/
                },
                        _ => {},
                    }
                }
                thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                for cell in  &generation {
                    if cell.alive {
                        canvas.set_draw_color(Color::RGB(0, 255, 0));
                    } else {
                        canvas.set_draw_color(Color::RGB(30, 30,30));
                    }
                    canvas.fill_rect(Rect::new(2*cell.x as i32, 2*cell.y as i32, cell_size, cell_size));
                }
                /*
                for (key,value) in &generation {

                    if *value {
                        canvas.set_draw_color(Color::RGB(0, 255, 0));

                    }else{
                        canvas.set_draw_color(Color::RGB(30, 30,30));
                    }

                    canvas.fill_rect(Rect::new(2*key.x as i32, 2*key.y as i32,cell_size, cell_size));

                }
                */


                canvas.present();


                generation = game_of_life::create_next_generation_concurrent(&generation, &life_rule);
                //generation = game_of_life::create_next_generation_concurrent_hash(&generation, &life_rule);
                // render window contents here
            }
        }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        let mut first_gen: Vec<game_of_life::CellCoordinates> = Vec::new();
        //let life_rule = setup::build_life_rule([false,false,false,true,false,false,true,false,false], [false,true,true,false,false,true,false,false,false]);
        //let mut generation= game_of_life::create_next_generation_concurrent(&first_gen, &life_rule);
        create_random_generation_vector(&mut first_gen);
        assert_eq!(300, first_gen.len());
    }
}
