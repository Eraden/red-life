use crate::backend::error::RLError;
use crate::backend::gamestate::GameState;
use crate::backend::screen::StackCommand;
use crate::backend::utils::get_scale;
use crate::game_core::event::Event;
use crate::game_core::resources::Resources;
use crate::machines::machine::Maschine;
use crate::machines::machine::State::Running;
use crate::machines::machine_sprite::MaschineSprite;
use crate::RLResult;
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, GameError};
use tracing::{error, info};

const MOVEMENT_SPEED: usize = 10;

impl GameState {
    pub fn move_player(&mut self, ctx: &mut Context) -> RLResult<StackCommand> {
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    info!("Escape pressed");
                    self.save(false)?;
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    if !self.collision_detection((
                        self.player.position.0,
                        self.player.position.1.saturating_sub(MOVEMENT_SPEED),
                    )) {
                        self.player.position.1 =
                            self.player.position.1.saturating_sub(MOVEMENT_SPEED);
                    }
                    //return Err(RLError::from(GameError::GraphicsInitializationError))
                    //error!("Player position: {:?}", RLError::Ui(GameError::GraphicsInitializationError));
                }
                VirtualKeyCode::A => {
                    if !self.collision_detection((
                        self.player.position.0.saturating_sub(MOVEMENT_SPEED),
                        self.player.position.1,
                    )) {
                        self.player.position.0 =
                            self.player.position.0.saturating_sub(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::S => {
                    if !self.collision_detection((
                        self.player.position.0,
                        self.player.position.1.saturating_add(MOVEMENT_SPEED),
                    )) {
                        self.player.position.1 =
                            self.player.position.1.saturating_add(MOVEMENT_SPEED);
                    }
                }
                VirtualKeyCode::D => {
                    if !self.collision_detection((
                        self.player.position.0.saturating_add(MOVEMENT_SPEED),
                        self.player.position.1,
                    )) {
                        self.player.position.0 =
                            self.player.position.0.saturating_add(MOVEMENT_SPEED);
                    }
                }
                // TODO: Interact with the possible area
                VirtualKeyCode::E => {
                    info!("In interaction area: {:?}", self.get_interactable());
                }
                _ => {}
            }
        }

        Ok(StackCommand::None)
    }
}
