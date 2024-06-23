use std::time::{Duration, SystemTime, SystemTimeError};

pub struct FrameLimiter {
    render_rate: u32,
    physical_rate: u32,
    render_interval: Duration,
    physics_interval: Duration,
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
            render_rate: render_rate,
            physical_rate: physics_rate,
            render_interval: Duration::from_nanos(1_000_000_000u64 / render_rate as u64),
            physics_interval: Duration::from_nanos(1_000_000_000u64 / physics_rate as u64),
            previous_render_time: SystemTime::now(),
            previous_physics_time: SystemTime::now(),
        }
    }

    pub fn step(&mut self) -> Result<FrameStepType, SystemTimeError> {
        loop {
            let render_elapsed_time = self.previous_render_time.elapsed()?;
            let physics_elapsed_time = self.previous_physics_time.elapsed()?;

            if render_elapsed_time >= self.render_interval
                && physics_elapsed_time >= self.physics_interval
            {
                self.previous_render_time = SystemTime::now();
                self.previous_physics_time = SystemTime::now();
                break Ok(FrameStepType::RenderAndPhysics);
            } else if render_elapsed_time >= self.render_interval {
                self.previous_render_time = SystemTime::now();
                break Ok(FrameStepType::Render);
            } else if physics_elapsed_time >= self.physics_interval {
                self.previous_physics_time = SystemTime::now();
                break Ok(FrameStepType::Physics);
            }

            std::thread::sleep(
                (self.render_interval - render_elapsed_time)
                    .min(self.physics_interval - physics_elapsed_time),
            );
        }
    }

    pub fn set_render_rate(&mut self, render_rate: u32) {
        self.render_rate = render_rate;
        self.render_interval = Duration::from_nanos(1_000_000_000u64 / render_rate as u64);
    }

    pub fn set_physics_rate(&mut self, physics_rate: u32) {
        self.physical_rate = physics_rate;
        self.physics_interval = Duration::from_nanos(1_000_000_000u64 / physics_rate as u64);
    }

    pub fn get_physics_rate(&self) -> u32 {
        self.physical_rate
    }

    pub fn get_rate_ratio(&self) -> f32 {
        self.physics_interval.as_secs_f32() / self.render_interval.as_secs_f32()
    }
}
