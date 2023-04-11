use crate::*;
use std::f32::consts::*;

//Range of the Field of view (1 -> full map width, 0 -> nothing)
const FOV_RANGE: f32 = 0.25;

//Angle of the Field of view in radians
const FOV_ANGLE: f32 = PI + FRAC_PI_4;

//Number of directions the eye is looking to
const CELLS: usize = 9;

#[derive(Debug)]
pub struct Eye {
    fov_range: f32,
    fov_angle: f32,
    cells: usize, //to divide the fov_angle in equal parts
}

impl Eye {
    fn new(fov_range: f32, fov_angle: f32, cells: usize) -> Self {
        assert!(fov_range > 0.0);
        assert!(fov_angle > 0.0);
        assert!(cells > 0);

        Self { fov_range, fov_angle, cells }
    }

    pub fn cells(&self) -> usize {
        self.cells
    }

    pub fn process_vision(
        &self,
        position: na::Point2<f32>,
        rotation: na::Rotation2<f32>,
        wastes: &[Waste],
    ) -> Vec<f32> {
        let mut cells = vec![0.0; self.cells];
        for waste in wastes {
            let vec = waste.position - position;

            //Check if waste is in range
            let dist = vec.norm();
            if dist >= self.fov_range {
                continue;
            }

            //Check if waste is in the vision span
            let angle = na::Rotation2::rotation_between(
                &na::Vector2::y(),
                &vec,
            ).angle();
            let angle = angle - rotation.angle();
            let angle = na::wrap(angle, -PI, PI);
            if angle < -self.fov_angle / 2.0 ||
               angle > self.fov_angle / 2.0
            {
                continue;
            }

            //Find corresponding cell index
            let angle = angle + self.fov_angle / 2.0;
            let cell = angle / self.fov_angle;
            let cell = cell * (self.cells as f32);
            let cell = (cell as usize).min(cells.len() - 1);

            //Compute energy required to collect this waste
            let energy = (self.fov_range - dist) / self.fov_range;
            cells[cell] += energy;
        }

        cells
    }
}

impl Default for Eye {
    fn default() -> Self {
        Self::new(FOV_RANGE, FOV_ANGLE, CELLS)
    }
}
