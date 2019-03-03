use std::env;
use std::process;

mod graphics_manager;
mod inputs_manager;
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

    let graphics_manager = graphics_manager::GraphicsManager::new();
    let _inputs_manager = inputs_manager::InputsManager::new();

    let mut chip = chip::Chip::new();
    let result = chip.load_game(&args[1]);
    match result {
        Err(e) => { println!("{:?}", e); process::exit(ERROR_GAME_LOADING_FAILED); },
        Ok(result) => result
    }

    loop {
        chip.emulate_cycle();
        if chip.draw_flag != 0 {
            graphics_manager.draw_graphics(chip.graphics);
        }

        //chip.set_keys(inputs_manager.get_inputs());

        if chip.exit_flag == 1 {
            break;
        }
    }
}
