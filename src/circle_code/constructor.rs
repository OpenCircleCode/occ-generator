/*
* Project: circle-code
* File: circle_code/consructor.rs
* Author: Quentin de Quelen (quentin@dequelen.me)
*/

use super::math;
use super::svg::{NB_POINTS, ANCHOR_EXT};

#[derive(Debug, Clone, Copy)]
pub struct Arc {
    pub start: u32,
    pub len: u32,
    pub level: u32
}

pub fn describe_arc(x: f64, y: f64, radius: f64, start_angle: f64, end_angle:f64) -> String {
    let start: math::CartesianCoord= math::polar_to_cartesian(x, y, radius, end_angle);
    let end: math::CartesianCoord = math::polar_to_cartesian(x, y, radius, start_angle);

    let large_arc_flag: &str = if end_angle - start_angle <= 180_f64 { "0" } else { "1" };

    format!("M {} {} A {} {} 0 {} 0 {} {}", start.x, start.y, radius, radius, large_arc_flag, end.x, end.y)
}


pub fn calculate_arcs(code: &[u32]) -> Vec<Arc> {
    let mut arcs: Vec<Arc> = Vec::new();

    let mut start: u32 = 0;
    let mut len:u32 = 0;
    let mut level:u32 = 0;

    let nb_points_for_anchor = (ANCHOR_EXT + 1_u32) / (360_u32 / NB_POINTS);
    let anchor_pos = NB_POINTS / 4;

    for c in code {
        let index = start + if len == 0 { 0 } else { len - 1 };
        if level == 2 && index % anchor_pos >= anchor_pos - nb_points_for_anchor {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
            }
            start += len + nb_points_for_anchor * 2;
            len = 0;
        } else if index >= NB_POINTS {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
            }
            len = 0;
            level += 1;
            start = if level == 2 {nb_points_for_anchor} else {0};
        } else if *c == 0 {
            if len != 0 {
                arcs.push(Arc{ start: start, len: len, level: level });
                start += len;
                len = 0;
            }
            start += 1;
        } else if *c == 1 {
            len += 1;
        }
    }

    arcs
}