use crate::Pixels;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SmoothScrollMode {
    Disabled,
    Interpolated,
}

#[derive(Clone, Debug)]
pub struct SmoothScrollState {
    pub mode: SmoothScrollMode,
    pub visual_offset: Pixels,
    pub target_offset: Pixels,
    pub animating: bool,
    pub last_frame_time: Option<Instant>,
    pub smoothing_factor: f32,
}

impl Default for SmoothScrollState {
    fn default() -> Self {
        Self {
            mode: SmoothScrollMode::Interpolated,
            visual_offset: Pixels::ZERO,
            target_offset: Pixels::ZERO,
            animating: false,
            last_frame_time: None,
            smoothing_factor: 0.18,
        }
    }
}

impl SmoothScrollState {
    pub fn set_mode(&mut self, mode: SmoothScrollMode) {
        self.mode = mode;

        if mode == SmoothScrollMode::Disabled {
            self.visual_offset = self.target_offset;
            self.animating = false;
        }
    }

    pub fn set_target(&mut self, target: Pixels) {
        self.target_offset = target;

        if self.mode == SmoothScrollMode::Disabled {
            self.visual_offset = target;
            self.animating = false;
            return;
        }

        if !self.animating {
            self.last_frame_time = Some(Instant::now());
        }

        self.animating = true;
    }

    pub fn update(&mut self) -> bool {
        if self.mode == SmoothScrollMode::Disabled {
            self.visual_offset = self.target_offset;
            self.animating = false;
            return false;
        }

        if !self.animating {
            return false;
        }

        let now = Instant::now();

        let dt = if let Some(last) = self.last_frame_time {
            now.duration_since(last).as_secs_f32()
        } else {
            0.016
        };

        self.last_frame_time = Some(now);

        let factor = 1.0 - (1.0 - self.smoothing_factor).powf(dt * 60.0);

        self.visual_offset.0 += (self.target_offset.0 - self.visual_offset.0) * factor;

        let remaining = (self.target_offset.0 - self.visual_offset.0).abs();

        if remaining < 0.5 {
            self.visual_offset = self.target_offset;
            self.animating = false;
            return false;
        }

        true
    }

    pub fn current(&self) -> Pixels {
        if self.mode == SmoothScrollMode::Disabled {
            self.target_offset
        } else {
            self.visual_offset
        }
    }
}
