use std::time::{Duration, SystemTime, SystemTimeError};

pub struct FrameLimiter {
    render_rate: u32,
    tick_rate: u32,
    render_interval: Duration,
    tick_interval: Duration,
    previous_render_time: SystemTime,
    previous_tick_time: SystemTime,
}

pub enum FrameStepType {
    Render,
    Tick,
    RenderAndTick,
}

impl FrameLimiter {
    pub fn new(render_rate: u32, tick_rate: u32) -> FrameLimiter {
        FrameLimiter {
            render_rate: render_rate,
            tick_rate: tick_rate,
            render_interval: Duration::from_nanos(1_000_000_000u64 / render_rate as u64),
            tick_interval: Duration::from_nanos(1_000_000_000u64 / tick_rate as u64),
            previous_render_time: SystemTime::now(),
            previous_tick_time: SystemTime::now(),
        }
    }

    pub fn step(&mut self) -> Result<FrameStepType, SystemTimeError> {
        loop {
            let render_elapsed_time = self.previous_render_time.elapsed()?;
            let tick_elapsed_time = self.previous_tick_time.elapsed()?;

            if render_elapsed_time >= self.render_interval
                && tick_elapsed_time >= self.tick_interval
            {
                self.previous_render_time = SystemTime::now();
                self.previous_tick_time = SystemTime::now();
                break Ok(FrameStepType::RenderAndTick);
            } else if render_elapsed_time >= self.render_interval {
                self.previous_render_time = SystemTime::now();
                break Ok(FrameStepType::Render);
            } else if tick_elapsed_time >= self.tick_interval {
                self.previous_tick_time = SystemTime::now();
                break Ok(FrameStepType::Tick);
            }

            std::thread::sleep(
                (self.render_interval - render_elapsed_time)
                    .min(self.tick_interval - tick_elapsed_time),
            );
        }
    }

    pub fn set_render_rate(&mut self, render_rate: u32) {
        self.render_rate = render_rate;
        self.render_interval = Duration::from_nanos(1_000_000_000u64 / render_rate as u64);
    }

    pub fn set_tick_rate(&mut self, tick_rate: u32) {
        self.tick_rate = tick_rate;
        self.tick_interval = Duration::from_nanos(1_000_000_000u64 / tick_rate as u64);
    }

    pub fn get_tick_rate(&self) -> u32 {
        self.tick_rate
    }

    pub fn get_rate_ratio(&self) -> f32 {
        self.tick_interval.as_secs_f32() / self.render_interval.as_secs_f32()
    }
}
