/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::svg_render::{
    BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTRE, Elements, LEFT, RIGHT, TOP, TOP_LEFT, TOP_RIGHT,
};

impl Elements {
    pub fn point_left_up(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn point_left_down(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn point_right_up(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
        ));
    }

    pub fn point_right_down(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
        ));
    }

    pub fn y_point_left(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn y_point_right(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
        ));
    }

    pub fn single_slip_point_up_up(coordinate: (i32, i32), svg: &mut String) {
        Self::crossing_up(coordinate, svg);
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP_RIGHT.0 + coordinate.0 - 5,
            TOP_RIGHT.1 + coordinate.1 + 5,
            LEFT.0 + coordinate.0 + 5,
            LEFT.1 + coordinate.1,
        ));
    }

    pub fn single_slip_point_up_down(coordinate: (i32, i32), svg: &mut String) {
        Self::crossing_up(coordinate, svg);
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM_LEFT.0 + coordinate.0 + 5,
            BOTTOM_LEFT.1 + coordinate.1 - 5,
            RIGHT.0 + coordinate.0 - 5,
            RIGHT.1 + coordinate.1,
        ));
    }

    pub fn single_slip_point_down_up(coordinate: (i32, i32), svg: &mut String) {
        Self::crossing_down(coordinate, svg);
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP_LEFT.0 + coordinate.0 + 5,
            TOP_LEFT.1 + coordinate.1 + 5,
            RIGHT.0 + coordinate.0 - 5,
            RIGHT.1 + coordinate.1,
        ));
    }

    pub fn single_slip_point_down_down(coordinate: (i32, i32), svg: &mut String) {
        Self::crossing_down(coordinate, svg);
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM_RIGHT.0 + coordinate.0 - 5,
            BOTTOM_RIGHT.1 + coordinate.1 - 5,
            LEFT.0 + coordinate.0 + 5,
            LEFT.1 + coordinate.1,
        ));
    }

    pub fn double_slip_point_up(coordinate: (i32, i32), svg: &mut String) {
        Self::crossing_up(coordinate, svg);
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP_RIGHT.0 + coordinate.0 - 5,
            TOP_RIGHT.1 + coordinate.1 + 5,
            LEFT.0 + coordinate.0 + 5,
            LEFT.1 + coordinate.1,
        ));
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM_LEFT.0 + coordinate.0 + 5,
            BOTTOM_LEFT.1 + coordinate.1 - 5,
            RIGHT.0 + coordinate.0 - 5,
            RIGHT.1 + coordinate.1,
        ));
    }

    pub fn double_slip_point_down(coordinate: (i32, i32), svg: &mut String) {
        Self::crossing_down(coordinate, svg);
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP_LEFT.0 + coordinate.0 + 5,
            TOP_LEFT.1 + coordinate.1 + 5,
            RIGHT.0 + coordinate.0 - 5,
            RIGHT.1 + coordinate.1,
        ));
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:0.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM_RIGHT.0 + coordinate.0 - 5,
            BOTTOM_RIGHT.1 + coordinate.1 - 5,
            LEFT.0 + coordinate.0 + 5,
            LEFT.1 + coordinate.1,
        ));
    }

    pub fn diagonal_point_up_right(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn diagonal_point_up_left(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn diagonal_point_down_right(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn diagonal_point_down_left(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn derailer_right_up(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn derailer_right_down(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn derailer_left_up(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn derailer_left_down(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::svg_render::Elements;

    use super::super::test_common::*;

    #[test]
    fn test_point_left_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::point_left_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "point-left-up");
    }

    #[test]
    fn test_point_left_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::point_left_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "point-left-down");
    }

    #[test]
    fn test_point_right_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::point_right_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "point-right-up");
    }

    #[test]
    fn test_point_right_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::point_right_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "point-right-down");
    }

    #[test]
    fn test_y_point_left() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::y_point_left(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "y-point-left");
    }

    #[test]
    fn test_y_point_right() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::y_point_right(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "y-point-right");
    }

    #[test]
    fn test_single_slip_point_up_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::single_slip_point_up_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "single-slip-point-up-up");
    }

    #[test]
    fn test_single_slip_point_up_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::single_slip_point_up_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "single-slip-point-up-down");
    }

    #[test]
    fn test_single_slip_point_down_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::single_slip_point_down_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "single-slip-point-down-up");
    }

    #[test]
    fn test_single_slip_point_down_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::single_slip_point_down_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "single-slip-point-down-down");
    }

    #[test]
    fn test_double_slip_point_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::double_slip_point_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "double-slip-point-up");
    }

    #[test]
    fn test_double_slip_point_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::double_slip_point_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "double-slip-point-down");
    }
}
