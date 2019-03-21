use std::env;
use std::process;
use std::time;
use std::thread;

mod runtime_manager;
mod chip;
mod opcodes;

const ERROR_INVALID_ARGUMENTS: i32 = 0x0001;
const ERROR_GAME_LOADING_FAILED: i32 = 0x0002;

const FRAME_PER_SECONDS: f32 = 10.0;
const MILLISECONDS_PER_FRAME: f32 = 1000.0 / FRAME_PER_SECONDS;

const REAL_WINDOW_WIDTH: u32 = 640;
const REAL_WINDOW_HEIGHT: u32 = 320;

fn display_usage_and_exit() {
    println!("Usage:");
    println!("./chip8 PATH_TO_GAME");
    process::exit(ERROR_INVALID_ARGUMENTS);
}

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        display_usage_and_exit();
    }

    // Initialize various runtime elements
    let mut runtime_manager = runtime_manager::RuntimeManager::new();
    let v = sfml::system::Vector2u::new(REAL_WINDOW_WIDTH, REAL_WINDOW_HEIGHT);
    runtime_manager.window.set_size(v);
    let mut chip = chip::Chip::new();
    let result = chip.load_game(&args[1]);
    match result {
        Err(e) => { println!("{:?}", e); process::exit(ERROR_GAME_LOADING_FAILED); },
        Ok(result) => result
    }

    let duration_per_frame: time::Duration = time::Duration::
                from_millis(MILLISECONDS_PER_FRAME.trunc() as u64);
    runtime_manager.draw_graphics(&chip.graphics);

    loop {
        let timer_start = time::SystemTime::now();

        loop {
            chip.emulate_cycle();
            runtime_manager.handle_events(&mut chip);

            if chip.clear_flag != 0 {
                runtime_manager.clear_screen();
            }

            if chip.draw_flag != 0 {
                runtime_manager.draw_graphics(&chip.graphics);
            }

            if chip.exit_flag == 1 {
                process::exit(0);
            }

            if chip.draw_flag != 0 || chip.clear_flag != 0 {
                let timer_end = time::SystemTime::now();
                let loop_time = timer_end.duration_since(timer_start).unwrap();
                if loop_time < duration_per_frame {
                    println!("[0000] SLEEPING   : {:?}", duration_per_frame - loop_time);
                    thread::sleep(duration_per_frame - loop_time);
                    chip.draw_flag = 0;
                    chip.clear_flag = 0;
                }
                else {
                    println!("Loop was too slow: {:?}", loop_time - duration_per_frame);
                }
                break;
            }
        }
    }
}
