use quicksilver::{
    geom::{Circle, Vector},
    graphics::{Color, VectorFont},
    input::{Event, Key},
    run, Graphics, Input, Result, Settings, Window,
};

use rand::Rng;
use lazy_static::lazy_static;
use std::sync::mpsc;
use std::sync::RwLock;
use std::{thread, time};


lazy_static! {
    static ref REACTION: RwLock<bool> = RwLock::new(false);
    static ref AIM: RwLock<bool> = RwLock::new(false);
    static ref CONTINUE: RwLock<bool> = RwLock::new(true);
}

fn main() {
    run (
        Settings {
            title: "Reaction & Aim Trainer",
            size: Vector{x: 800.0, y: 600.0},
            resizable: false,
            ..Settings::default()
        },
        home,
    );


    // println!("welcome to the reaction & aim tester!");
    // println!("Enter [R]eaction if you would like to test your reaction or [A] if you would like to test your aim.");
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).ok();
    // let input = input.trim();

    // if input == "r" {
    //     println!("You have chosen to test your reaction time!");
    //     run(
    //         Settings {
    //             size: Vector { x: 800.0, y: 600.0 },
    //             title: "Reaction Timer",
    //             ..Settings::default()
    //         },
    //         reaction_time,
    //     );
    // } else if input == "a" {
    //     println!("You have chosen to test your aim!");
    //     run(
    //         Settings {
    //             size: Vector { x: 800.0, y: 600.0 },
    //             title: "Aim trainer",
    //             ..Settings::default()
    //         },
    //         aim_trainer,
    //     );
    // } else {
    //     println!("You have entered an invalid input. Please try again.");
    //     main();
    // }
}

async fn home(window: Window, mut gfx: Graphics, mut input: Input) -> Result<()> {
    // while *CONTINUE.read().unwrap() {

    // }
    
    // let mut running = true;
    while *CONTINUE.read().unwrap() {
        gfx.clear(Color::WHITE);
        // gfx.present(&window)?;
        let ttf = VectorFont::load("../static/Exo2.ttf").await.unwrap();
        // let w = &window;
        let mut font = ttf.to_renderer(&gfx, 32.0)?;
    
        font.draw_wrapping(
            &mut gfx,
            "Welcome to the reaction & aim tester!",
            Some(500.0),
            Color::BLACK,
            Vector::new(100.0, 300.0),
        )?;

        gfx.present(&window)?;
        thread::sleep(time::Duration::from_millis(2000));

        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) => {
                    if key.key() == Key::Escape {
                        *CONTINUE.write().unwrap() = false;
                        // running = false;
                    }
                    if key.key() == Key::R {
                        println!("You have chosen to test your reaction time!");
                        // running = false;
                        let mut reaction_bool = REACTION.write().unwrap();
                        *reaction_bool = true;
                        reaction_time(&window, &mut gfx, &mut input).await?;
                        break;
                    }
                    if key.key() == Key::A {
                        println!("You have chosen to test your aim!");
                        // running = false;
                        let mut aim_bool = AIM.write().unwrap();
                        *aim_bool = true;
                        aim_trainer(&window, &mut gfx, &mut input).await?;
                        break;
                    }
                }
                _ => {}
            }
        } 
    }
    
    // if *REACTION.read().unwrap() {
    //     reaction_time(window, gfx, input).await?;
    // } else if *AIM.read().unwrap() {
    //     aim_trainer(window, gfx, input).await?;
    // }

    Ok(())
}

// #[async_recursion]
async fn reaction_time(window: &Window, gfx: &mut Graphics, input: &mut Input) -> Result<()> {
    gfx.clear(Color::WHITE);
    gfx.present(&window)?;
    let ttf = VectorFont::load("Exo2.ttf").await?;
    let mut font = ttf.to_renderer(&gfx, 40.0)?;
    
    // let (send, _recv) = mpsc::channel();
    let _timer = thread::spawn(move || {
        let sleep_time = rand::thread_rng().gen_range(0..5) + 10;
        thread::sleep(time::Duration::from_secs(sleep_time));
        // send.send(true).unwrap();
    });

    gfx.clear(Color::GREEN);
    gfx.present(&window)?;
    let start_time = time::SystemTime::now();
    
    let mut running = true;
    while running {
        while let Some(event) = input.next_event().await {
            match event {
                Event::KeyboardInput(key) => {
                    if key.key() == Key::Space {
                        running = false;
                        let end_time = time::SystemTime::now();
                        let duration = end_time.duration_since(start_time).unwrap();
                        let duration_ms = duration.as_millis();
                        
                        gfx.clear(Color::WHITE);
                        println!("Your reaction time was {}ms", duration_ms);
                        font.draw_wrapping(
                            gfx,
                            &format!("Your reaction time was {}ms", duration_ms),
                            Some(500.0),
                            Color::BLACK,
                            Vector::new(170.0, 300.0),
                        )?;
                        gfx.present(&window)?;
                        thread::sleep(time::Duration::from_secs(2));
                    }
                }
                _ => {}
            }
        }
    }
    
    let mut c = CONTINUE.write().unwrap();
    *c = true;
    
    Ok(())
}

async fn aim_trainer(window: &Window, gfx: &mut Graphics, input: &mut Input) -> Result<()> {
    let mut rand_pos = Vector::new(0.0, 0.0);
    let mut target_exists = false;
    let mut count = 0;
    let mut average_time = 0.0;
    let mut start_time = time::SystemTime::now();

    loop {
        while let Some(_) = input.next_event().await {}
        gfx.clear(Color::WHITE);
        if !target_exists {
            rand_pos = Vector::new(
                rand::thread_rng().gen_range(50..750) as f32,
                rand::thread_rng().gen_range(50..550) as f32,
            );
            gfx.fill_circle(&Circle::new(rand_pos, 20.0), Color::RED);
            target_exists = true;
            count = count + 1;
            start_time = time::SystemTime::now();
        } else {
            gfx.fill_circle(&Circle::new(rand_pos, 20.0), Color::RED);
        }
        let mouse = gfx.screen_to_camera(&window, input.mouse().location());
        if mouse.distance(rand_pos) < 10.0 {
            target_exists = false;
            gfx.fill_circle(&Circle::new(rand_pos, 20.0), Color::WHITE);
            let end_time = time::SystemTime::now();
            println!(
                "{}",
                end_time.duration_since(start_time).unwrap().as_millis()
            );
            average_time += end_time.duration_since(start_time).unwrap().as_millis() as f32;
        }
        gfx.fill_circle(&Circle::new(mouse, 20.0), Color::RED);
        if count == 10 {
            average_time = average_time / 10.0;
            println!("Average reaction time: {} ms", average_time);
            break;
        }
        gfx.present(&window)?;
    }
    Ok(())
}
