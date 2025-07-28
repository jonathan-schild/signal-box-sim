/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::svg_render::{
    BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTRE, Elements, LEFT, RIGHT, TOP, TOP_LEFT, TOP_RIGHT,
};

impl Elements {
    pub fn track_horizontal(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
        ));
    }

    pub fn track_vertical(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM.0 + coordinate.0,
            BOTTOM.1 + coordinate.1,
            TOP.0 + coordinate.0,
            TOP.1 + coordinate.1,
        ));
    }

    pub fn track_up(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn track_down(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn curve_left_up(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn curve_left_down(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn curve_right_up(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
        ));
    }

    pub fn curve_right_down(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
        ));
    }

    pub fn curve_top_left(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            TOP.0 + coordinate.0,
            TOP.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
        ));
    }

    pub fn curve_top_right(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            TOP.0 + coordinate.0,
            TOP.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn curve_bottom_left(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            BOTTOM.0 + coordinate.0,
            BOTTOM.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
        ));
    }

    pub fn curve_bottom_right(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {} {}, {}"
    />
"#,
            BOTTOM.0 + coordinate.0,
            BOTTOM.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::svg_render::Elements;

    use super::super::test_common::*;

    #[test]
    fn test_track_horizontal() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::track_horizontal(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "track-horizontal");
    }

    #[test]
    fn test_track_vertical() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::track_vertical(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "track-vertical");
    }

    #[test]
    fn test_track_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::track_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "track-up");
    }

    #[test]
    fn test_track_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::track_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "track-down");
    }

    #[test]
    fn test_curve_left_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_left_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-left-up");
    }

    #[test]
    fn test_curve_left_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_left_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-left-down");
    }

    #[test]
    fn test_curve_right_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_right_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-right-up");
    }

    #[test]
    fn test_curve_right_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_right_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-right-down");
    }

    #[test]
    fn test_curve_top_left() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_top_left(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-top-left");
    }

    #[test]
    fn test_curve_top_right() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_top_right(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-top-right");
    }

    #[test]
    fn test_curve_bottom_left() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_bottom_left(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-bottom-left");
    }

    #[test]
    fn test_curve_bottom_right() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::curve_bottom_right(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "curve-bottom-right");
    }
}
