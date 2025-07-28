/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

mod crossing_and_tunnels;
mod points_and_derailer;
mod track_and_curves;

pub type Coordinate = (i32, i32);

const SCALE: i32 = 20;

const TOP_LEFT: Coordinate = (0, 0);
const TOP: Coordinate = (SCALE / 2, 0);
const TOP_RIGHT: Coordinate = (SCALE, 0);
const RIGHT: Coordinate = (SCALE, SCALE / 2);
const BOTTOM_RIGHT: Coordinate = (SCALE, SCALE);
const BOTTOM: Coordinate = (SCALE / 2, SCALE);
const BOTTOM_LEFT: Coordinate = (0, SCALE);
const LEFT: Coordinate = (0, SCALE / 2);
const CENTRE: Coordinate = (SCALE / 2, SCALE / 2);

pub type ID = u32;

pub type ElementDescriptor = (Elements, ID);

pub enum Elements {
    // Tracks
    TrackHorizontal,
    TrackVertical,
    TrackUp,
    TrackDown,
    // Curves
    CurveLeftUp,
    CurveLeftDown,
    CurveRightUp,
    CurveRightDown,
    CurveTopLeft,
    CurveTopRight,
    CurveBottomLeft,
    CurveBottomRight,
    // Crossings
    DiagonalCrossing,
    CrossingUp,
    CrossingDown,
    // Points
    PointLeftUp,
    PointLeftDown,
    PointRightUp,
    PointRightDown,
    YPointRight,
    YPointLeft,
    SingleSlipPointUpUp,
    SingleSlipPointUpDown,
    SingleSlipPointDownUp,
    SingleSlipPointDownDown,
    DoubleSlipPointUp,
    DoubleSlipPointDown,
    DiagonalPointUpRight,
    DiagonalPointUpLeft,
    DiagonalPointDownRight,
    DiagonalPointDownLeft,
    // Derailer
    DerailerRightUp,
    DerailerRightDown,
    DerailerLeftUp,
    DerailerLeftDown,
    // Tunnel
    DiagonalTunnelUp,
    DiagonalTunnelDown,
    TunnelHorizontal,
    TunnelVertical,
    None,
}

impl Elements {
    pub fn render_svg(&self, coordinate: (u16, u16), svg: &mut String) {
        let coordinate = (coordinate.0 as i32 * SCALE, coordinate.1 as i32 * SCALE);

        match self {
            Elements::TrackHorizontal => Self::track_horizontal(coordinate, svg),
            Elements::TrackVertical => Self::track_vertical(coordinate, svg),
            Elements::TrackUp => Self::track_up(coordinate, svg),
            Elements::TrackDown => Self::track_down(coordinate, svg),
            Elements::CurveLeftUp => Self::curve_left_up(coordinate, svg),
            Elements::CurveLeftDown => Self::curve_left_down(coordinate, svg),
            Elements::CurveRightUp => Self::curve_right_up(coordinate, svg),
            Elements::CurveRightDown => Self::curve_right_down(coordinate, svg),
            Elements::CurveTopLeft => Self::curve_top_left(coordinate, svg),
            Elements::CurveTopRight => Self::curve_top_right(coordinate, svg),
            Elements::CurveBottomLeft => Self::curve_bottom_left(coordinate, svg),
            Elements::CurveBottomRight => Self::curve_bottom_right(coordinate, svg),
            Elements::DiagonalCrossing => Self::diagonal_crossing(coordinate, svg),
            Elements::CrossingUp => Self::crossing_up(coordinate, svg),
            Elements::CrossingDown => Self::crossing_down(coordinate, svg),
            Elements::PointLeftUp => Self::point_left_up(coordinate, svg),
            Elements::PointLeftDown => Self::point_left_down(coordinate, svg),
            Elements::PointRightUp => Self::point_right_up(coordinate, svg),
            Elements::PointRightDown => Self::point_right_down(coordinate, svg),
            Elements::YPointRight => Self::y_point_left(coordinate, svg),
            Elements::YPointLeft => Self::y_point_right(coordinate, svg),
            Elements::SingleSlipPointUpUp => Self::single_slip_point_up_up(coordinate, svg),
            Elements::SingleSlipPointUpDown => Self::single_slip_point_up_down(coordinate, svg),
            Elements::SingleSlipPointDownUp => Self::single_slip_point_down_up(coordinate, svg),
            Elements::SingleSlipPointDownDown => Self::single_slip_point_down_down(coordinate, svg),
            Elements::DoubleSlipPointUp => Self::double_slip_point_up(coordinate, svg),
            Elements::DoubleSlipPointDown => Self::diagonal_point_up_right(coordinate, svg),
            Elements::DiagonalPointUpRight => Self::diagonal_point_down_right(coordinate, svg),
            Elements::DiagonalPointUpLeft => Self::diagonal_point_up_left(coordinate, svg),
            Elements::DiagonalPointDownRight => Self::diagonal_point_down_right(coordinate, svg),
            Elements::DiagonalPointDownLeft => Self::diagonal_point_down_left(coordinate, svg),
            Elements::DerailerRightUp => Self::derailer_right_up(coordinate, svg),
            Elements::DerailerRightDown => Self::derailer_right_down(coordinate, svg),
            Elements::DerailerLeftUp => Self::derailer_left_up(coordinate, svg),
            Elements::DerailerLeftDown => Self::derailer_left_down(coordinate, svg),
            Elements::DiagonalTunnelUp => Self::diagonal_tunnel_up(coordinate, svg),
            Elements::DiagonalTunnelDown => Self::diagonal_tunnel_down(coordinate, svg),
            Elements::TunnelHorizontal => Self::tunnel_horizontal(coordinate, svg),
            Elements::TunnelVertical => Self::tunnel_vertical(coordinate, svg),
            Elements::None => (),
        }
    }
}

#[cfg(test)]
mod test_common {
    use std::{
        fs::{OpenOptions, create_dir_all},
        io::{BufWriter, Write},
    };

    use super::SCALE;

    pub const NULL: (i32, i32) = (0, 0);

    pub fn svg_prolog(svg: &mut String) {
        svg.push_str(&format!(
            r#"<svg
    version="1.1"
    width="{SCALE}mm"
    height="{SCALE}mm"
    viewBox="0 0 {SCALE} {SCALE}"
    xmlns="http://www.w3.org/2000/svg">
"#
        ));
    }

    pub fn svg_epilog(svg: &mut String) {
        svg.push_str(r#"</svg>"#);
    }

    pub fn write(svg: String, file: &str) {
        create_dir_all("target/test/").unwrap();
        let mut file = BufWriter::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(format!("target/test/{}.svg", file))
                .unwrap(),
        );
        file.write_all(svg.as_bytes()).unwrap();
    }
}

#[cfg(test)]
mod test {

    use crate::svg_render::{
        Elements,
        test_common::{svg_epilog, write},
    };

    #[test]
    fn test_render_circle() {
        let mut svg = String::new();
        svg.push_str(
            r#"<svg
    version="1.1"
    width="140mm"
    height="120mm"
    viewBox="0 0 140 120"
    xmlns="http://www.w3.org/2000/svg">
"#,
        );

        let elements = [
            [
                Elements::None,
                Elements::None,
                Elements::CurveRightDown,
                Elements::TrackHorizontal,
                Elements::CurveLeftDown,
                Elements::None,
                Elements::None,
            ],
            [
                Elements::None,
                Elements::TrackUp,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::TrackDown,
                Elements::None,
            ],
            [
                Elements::CurveBottomRight,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::CurveBottomLeft,
            ],
            [
                Elements::CurveTopRight,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::TrackVertical,
            ],
            [
                Elements::None,
                Elements::CurveRightUp,
                Elements::TrackHorizontal,
                Elements::CurveLeftDown,
                Elements::None,
                Elements::None,
                Elements::CurveTopLeft,
            ],
            [
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::None,
                Elements::CurveRightUp,
                Elements::CurveLeftUp,
                Elements::None,
            ],
        ];

        for (y, es) in elements.iter().enumerate() {
            for (x, e) in es.iter().enumerate() {
                e.render_svg((x as u16, y as u16), &mut svg);
            }
        }

        svg_epilog(&mut svg);
        write(svg, "A-svg-circle");
    }
}
