use crate::screen::StackCommand;
use crate::utils::get_scale;
use crate::{screen::Screen, RedResult};
use ggez::winit::event::VirtualKeyCode;
use ggez::{graphics, Context};
use serde::{Deserialize, Serialize};

const MOVEMENT_SPEED: usize = 5;

#[derive(Clone, Eq, Debug, PartialEq, Serialize, Deserialize)]
struct Item;

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct Player {
    position: (usize, usize),
}

#[derive(Clone, Default, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    inventory: Vec<Item>,
    player: Player,
    milestone: usize,
}

impl GameState {
    pub fn tick(&mut self) {
        // do stuff
        self.milestone += 1;
    }

    fn save(&self, milestone: bool) -> RedResult {
        let save_data = serde_yaml::to_string(self)?;
        // Create the folder if it doesn't exist
        std::fs::create_dir_all("./saves")?;
        if milestone {
            std::fs::write("./saves/milestone.yaml", save_data)?;
        } else {
            std::fs::write("./saves/autosave.yaml", save_data)?;
        }
        Ok(())
    }

    pub fn load(milestone: bool) -> RedResult<GameState> {
        let save_data = if milestone {
            std::fs::read_to_string("./saves/milestone.yaml")?
        } else {
            std::fs::read_to_string("./saves/autosave.yaml")?
        };
        let game_state = serde_yaml::from_str(&save_data)?;
        Ok(game_state)
    }
}

impl Screen for GameState {
    fn update(&mut self, ctx: &mut Context) -> RedResult<StackCommand> {
        self.tick();
        let keys = ctx.keyboard.pressed_keys();
        for key in keys.iter() {
            match key {
                VirtualKeyCode::Escape => {
                    self.save(false)?;
                    return Ok(StackCommand::Pop);
                }
                VirtualKeyCode::W => {
                    self.player.position.1 = self.player.position.1.saturating_sub(MOVEMENT_SPEED);
                }
                VirtualKeyCode::A => {
                    self.player.position.0 = self.player.position.0.saturating_sub(MOVEMENT_SPEED);
                }
                VirtualKeyCode::S => {
                    self.player.position.1 = self.player.position.1.saturating_add(MOVEMENT_SPEED);
                }
                VirtualKeyCode::D | VirtualKeyCode::Right => {
                    self.player.position.0 = self.player.position.0.saturating_add(MOVEMENT_SPEED);
                }
                key => {
                    dbg!("{:?}", key);
                }
            }
        }
        Ok(StackCommand::None)
    }

    fn draw(&self, ctx: &mut Context) -> RedResult {
        let scale = get_scale(ctx);
        let mut canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.2, 0.3, 1.0]));
        // TODO: After asset loading we can load this from a hashmap: let background = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/basis.png"))?;
        //canvas.draw(&background, graphics::DrawParam::default().scale(scale));
        let player = graphics::Image::from_bytes(ctx, include_bytes!("../../assets/player.png"))?;
        canvas.draw(
            &player,
            graphics::DrawParam::default()
                .scale(scale)
                .dest([self.player.position.0 as f32, self.player.position.1 as f32]),
        );
        canvas.finish(ctx)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_gamestate() {
        let gamestate = GameState::default();
    }

    #[test]
    fn test_save_autosave() {
        let gamestate = GameState::default();
        gamestate.save(false).unwrap();
    }

    #[test]
    fn test_save_milestone() {
        let gamestate = GameState::default();
        gamestate.save(true).unwrap();
    }

    #[test]
    fn test_load_autosave() {
        let gamestate = GameState::load(false).unwrap();
    }

    #[test]
    fn test_load_milestone() {
        let gamestate = GameState::load(true).unwrap();
    }
}
