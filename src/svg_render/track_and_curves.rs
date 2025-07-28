/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::svg_render::{BOTTOM, Elements, LEFT, RIGHT, TOP};

impl Elements {
    pub fn track_horizontal(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"<path
                style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
                d="M {}, {} {} {}"
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
            r#"<path
                style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
                d="M {}, {} {} {}"
            />
            "#,
            BOTTOM.0 + coordinate.0,
            BOTTOM.1 + coordinate.1,
            TOP.0 + coordinate.0,
            TOP.1 + coordinate.1,
        ));
    }

    pub fn track_up(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn track_down(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_left_up(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_left_down(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_right_up(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_right_down(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_top_left(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_top_right(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_bottom_left(coordinate: (i32, i32), svg: &mut String) {
        todo!()
    }

    pub fn curve_bottom_right(coordinate: (i32, i32), svg: &mut String) {
        todo!()
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
}
