//! This file contains constants that are necessary for the game.
use crate::backend::rlcolor::RLColor;
use crate::backend::utils::gen_inventory;
use crate::game_core::item::Item;
use crate::game_core::resources::Resources;
use crate::languages::german::{BENZIN, GEDRUCKTESTEIL};
use crate::machines::machine::State;
use crate::machines::trade::Trade;
use ggez::graphics::{Color, Rect};
use std::string::ToString;

/// Contains the screen resolution of the game.
pub const SCREEN_RESOLUTION: (f32, f32) = (1920., 1080.);

/// Contains the desired FPS of the game-loop.
pub(crate) const DESIRED_FPS: u32 = 60;

/// Contains the map border( x-right, y-bottom, x-left, y-top)
pub const MAP_BORDER: [usize; 4] = [1780, 860, 270, 220];

/// Contains the position of the resource bars.
pub(crate) const RESOURCE_POSITION: [f32; 3] = [316.0, 639.0, 1373.0];

/// Contains the color used for the resource bars.
pub(crate) const COLORS: [Color; 3] = [RLColor::BLUE, RLColor::GOLD, RLColor::DARK_RED];

/// Contains the size of the player icon to scale the collision area.
pub(crate) const PLAYER_ICON_SIZE: (usize, usize) = (58, 96);

/// Contains the interaction radius of the player.
pub(crate) const PLAYER_INTERACTION_RADIUS: f32 = 50.;
// pub const MACHINE_POSITIONS: [[i32; 4]; 4] = [[0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0], [0, 0, 0, 0]];

//"its a const lol" // problem ist das ich nicht vec![] in const aufrufen darf und es wäre anstrengend
#[allow(clippy::too_many_lines)]
pub(crate) fn gen_all_machines() -> [(String, Rect, Vec<Trade>, Resources<i16>); 6] {
    [
        //("BEISPIEL".to_string() , Rect::default(), vec![Trade::default()], Resources::default()),
        (
            "test".to_string(),
            Rect {
                x: 284.0,
                y: 230.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "free_items".to_string(),
                    0,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(-100, -100, -100),
                ),
                Trade::new(
                    "reset_items".to_string(),
                    0,
                    State::Idle,
                    State::Running,
                    false,
                    gen_inventory(100, 97, 99),
                ),
                Trade::new(
                    "set_future_start_items".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    false,
                    gen_inventory(-100, -100, -100),
                ),
            ],
            Resources {
                oxygen: -25,
                energy: -25,
                life: -4,
            },
        ),
        /////////////////////////////////////////////////////////////////////////////////////////////
        (
            "Oxygen".to_string(),
            Rect {
                x: 600.0,
                y: 250.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_o2".to_string(),
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0),
                ),
                Trade::new(
                    "start_02".to_string(),
                    0,
                    State::Idle,
                    State::Running,
                    false,
                    gen_inventory(0, 0, 0),
                ),
                Trade::new(
                    "stop_02".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 0),
                ),
            ],
            Resources {
                oxygen: 20,
                energy: -30,
                life: 0,
            },
        ),
        /////////////////////////////////////////////////////////////////////////////////////////////
        (
            "Stromgenerator".to_string(),
            Rect {
                x: 284.0,
                y: 740.0,
                w: 200.0,
                h: 200.0,
            },
            vec![
                Trade::new(
                    "Stromgenerator_fueld".to_string(),
                    1000,
                    State::Broken,
                    State::Running,
                    true,
                    gen_inventory(0, 1, 0),
                ),
                Trade::new(
                    "Stromgenerator_Starten".to_string(),
                    1,
                    State::Idle,
                    State::Running,
                    false,
                    gen_inventory(0, 0, 0),
                ),
                Trade::new(
                    "Stromgenerator_Pausiert".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 0),
                ),
            ],
            Resources {
                oxygen: -5,
                energy: 50,
                life: 0,
            },
        ),
        /////////////////////////////////////////////////////////////////////////////////////////////
        (
            "werkermaschine".to_string(),
            Rect {
                x: 600.0,
                y: 600.0,
                w: 200.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_test".to_string(),
                    100,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 1),
                ),
                Trade::new(
                    "repair_test".to_string(),
                    120,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(-1, 0, 0),
                ), /*
                   Trade::new(
                       "repair_test".to_string(),
                       100,
                       State::Running,
                       State::Idle,
                       gen_inventory(0, 0, 0),
                   ),*/
            ],
            Resources {
                oxygen: 0,
                energy: -15,
                life: 0,
            },
        ),
        /////////////////////////////////////////////////////////////////////////////////////////////
        (
            "3d_printer".to_string(),
            Rect {
                x: 1722.0,
                y: 840.0,
                w: 100.0,
                h: 100.0,
            },
            vec![
                Trade::new(
                    "repair_3d".to_string(),
                    300,
                    State::Broken,
                    State::Idle,
                    false,
                    gen_inventory(2, 1, 0),
                ),
                Trade::new(
                    "print_3d_part".to_string(),
                    200,
                    State::Idle,
                    State::Running,
                    true,
                    gen_inventory(2, 0, -1),
                ),/*
                Trade::new(
                    "pause_3d_print".to_string(),
                    0,
                    State::Running,
                    State::Idle,
                    false,
                    gen_inventory(0, 0, 0),
                ),*/
            ],
            Resources {
                oxygen: 0,
                energy: -25,
                life: 0,
            },
        ),
        /////////////////////////////////////////////////////////////////////////////////////////////
        (
            "Loch".to_string(),
            Rect {
                x: 1722.0,
                y: 230.0,
                w: 100.0,
                h: 100.0,
            },
            vec![/*
                Trade::new(
                    "loch".to_string(),
                    0,
                    State::Broken,
                    State::Idle,
                    gen_inventory(2, 2, 2),
                    Item::new(BENZIN),
                    0,
                ),
                Trade::new(
                    "repair_test".to_string(),
                    0,
                    State::Idle,
                    State::Running,
                    gen_inventory(0, 1, 2),
                    Item::new(BENZIN),
                    0,
                ),*/
                Trade::new(
                    "Loch_reparien".to_string(),
                    100,
                    State::Running,
                    State::Idle,
                    false,
                    gen_inventory(2, 0, 0),
                ),
            ],
            Resources {
                oxygen: -20,
                energy: -5,
                life: -2,
            },
        ),
    ]
}
