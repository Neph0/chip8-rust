use std::env;
use std::process;

mod runtime_manager;
mod chip;

const ERROR_INVALID_ARGUMENTS: i32 = 0x0001;
const ERROR_GAME_LOADING_FAILED: i32 = 0x0002;

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
    let mut chip = chip::Chip::new(runtime_manager);
    let result = chip.load_game(&args[1]);
    match result {
        Err(e) => { println!("{:?}", e); process::exit(ERROR_GAME_LOADING_FAILED); },
        Ok(result) => result
    }

    loop {
        chip.emulate_cycle();

        chip.handle_events();
        //runtime_manager.handle_events(&mut chip);
        if chip.draw_flag != 0 {
            chip.draw_graphics();
            //runtime_manager.draw_graphics(&chip.graphics);
        }

        if chip.exit_flag == 1 {
            break;
        }
    }
}
