use std::time::{Duration, SystemTime, SystemTimeError};

pub struct FrameLimiter {
    render_rate: Duration,
    physics_rate: Duration,
    previous_render_time: SystemTime,
    previous_physics_time: SystemTime,
}

pub enum FrameStepType {
    Render,
    Physics,
    RenderAndPhysics,
}

impl FrameLimiter {
    pub fn new(render_rate: u32, physics_rate: u32) -> FrameLimiter {
        FrameLimiter {
            render_rate: Duration::from_nanos(1_000_000_000u64 / render_rate as u64),
            physics_rate: Duration::from_nanos(1_000_000_000u64 / physics_rate as u64),
            previous_render_time: SystemTime::now(),
            previous_physics_time: SystemTime::now(),
        }
    }

    pub fn step(&mut self) -> Result<FrameStepType, SystemTimeError> {
        loop {
            let render_elapsed_time = self.previous_render_time.elapsed()?;
            let physics_elapsed_time = self.previous_physics_time.elapsed()?;

            if render_elapsed_time >= self.render_rate && physics_elapsed_time >= self.physics_rate
            {
                self.previous_render_time = SystemTime::now();
                self.previous_physics_time = SystemTime::now();
                break Ok(FrameStepType::RenderAndPhysics);
            } else if render_elapsed_time >= self.render_rate {
                self.previous_render_time = SystemTime::now();
                break Ok(FrameStepType::Render);
            } else if physics_elapsed_time >= self.physics_rate {
                self.previous_physics_time = SystemTime::now();
                break Ok(FrameStepType::Physics);
            }

            std::thread::sleep(
                (self.render_rate - render_elapsed_time)
                    .min(self.physics_rate - physics_elapsed_time),
            );
        }
    }
}
