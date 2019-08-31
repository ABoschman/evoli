use amethyst::{
    core::math::Vector2,
    ecs::*,
    input::{InputEvent, StringBindings},
    shrev::{EventChannel, ReaderId},
};

use crate::resources::wind::Wind;
use std::f32;

/// DebugWindControlSystem allows players to change the wind direction at runtime.
/// Wind direction will rotate counter-clockwise by 1/8 PI RAD every time the
/// ChangeWindDirection input action is invoked.
/// The magnitude of the wind vector will remain unchanged.
#[derive(Default)]
pub struct DebugWindControlSystem {
    input_reader_id: Option<ReaderId<InputEvent<StringBindings>>>,
}

impl DebugWindControlSystem {
    fn handle_action(&self, action: &str, wind: &mut Wind) {
        match action {
            "ChangeWindDirection" => {
                let old_wind_angle = wind.wind.y.atan2(wind.wind.x);
                let new_wind_angle = old_wind_angle + f32::consts::FRAC_PI_8;
                let magnitude = wind.wind.magnitude();
                wind.wind = Vector2::new(
                    magnitude * new_wind_angle.cos(),
                    magnitude * new_wind_angle.sin(),
                );
                println!(
                    "action: {:?} Changed wind angle from {:?} to {:?}",
                    action, old_wind_angle, new_wind_angle
                );
            }
            _ => (),
        }
    }
}

impl<'s> System<'s> for DebugWindControlSystem {
    type SystemData = (
        Read<'s, EventChannel<InputEvent<StringBindings>>>,
        Write<'s, Wind>,
    );

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);
        self.input_reader_id = Some(
            res.fetch_mut::<EventChannel<InputEvent<StringBindings>>>()
                .register_reader(),
        );
    }

    fn run(&mut self, (input_events, mut wind): Self::SystemData) {
        input_events
            .read(self.input_reader_id.as_mut().unwrap())
            .for_each(|event| {
                // change from if-let to match when more InputEvent variants need to be handled
                if let InputEvent::ActionPressed(action_name) = event {
                    self.handle_action(&action_name, &mut wind);
                }
            });
    }
}
