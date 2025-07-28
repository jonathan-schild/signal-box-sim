/*
 * Copyright (c) 2025 Jonathan "Nath" Schild. Licensed under the EUPL-1.2
 */

use crate::svg_render::{
    BOTTOM, BOTTOM_LEFT, BOTTOM_RIGHT, CENTRE, Elements, LEFT, RIGHT, TOP, TOP_LEFT, TOP_RIGHT,
};

impl Elements {
    pub fn diagonal_crossing(coordinate: (i32, i32), svg: &mut String) {
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
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
        ));
    }

    pub fn crossing_up(coordinate: (i32, i32), svg: &mut String) {
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
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
        ));
    }

    pub fn crossing_down(coordinate: (i32, i32), svg: &mut String) {
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
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
        ));
    }

    pub fn tunnel_horizontal(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
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
            TOP.0 + coordinate.0,
            TOP.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1 - CENTRE.1 / 3,
            BOTTOM.0 + coordinate.0,
            BOTTOM.1 + coordinate.1,
            CENTRE.0 + coordinate.0,
            CENTRE.1 + coordinate.1 + CENTRE.1 / 3,
        ));
    }

    pub fn tunnel_vertical(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP.0 + coordinate.0,
            TOP.1 + coordinate.1,
            BOTTOM.0 + coordinate.0,
            BOTTOM.1 + coordinate.1,
            LEFT.0 + coordinate.0,
            LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0 - CENTRE.0 / 3,
            CENTRE.1 + coordinate.1,
            RIGHT.0 + coordinate.0,
            RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0 + CENTRE.0 / 3,
            CENTRE.1 + coordinate.1,
        ));
    }

    pub fn diagonal_tunnel_up(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0 - CENTRE.0 / 3,
            CENTRE.1 + coordinate.1 - CENTRE.0 / 3,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0 + CENTRE.0 / 3,
            CENTRE.1 + coordinate.1 + CENTRE.0 / 3,
        ));
    }

    pub fn diagonal_tunnel_down(coordinate: (i32, i32), svg: &mut String) {
        svg.push_str(&format!(
            r#"    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
    <path
        style="stroke:#000000;stroke-opacity:1;fill:none;stroke-width:1.5"
        d="M {}, {} {}, {}"
    />
"#,
            TOP_LEFT.0 + coordinate.0,
            TOP_LEFT.1 + coordinate.1,
            BOTTOM_RIGHT.0 + coordinate.0,
            BOTTOM_RIGHT.1 + coordinate.1,
            BOTTOM_LEFT.0 + coordinate.0,
            BOTTOM_LEFT.1 + coordinate.1,
            CENTRE.0 + coordinate.0 - CENTRE.0 / 3,
            CENTRE.1 + coordinate.1 + CENTRE.1 / 3,
            TOP_RIGHT.0 + coordinate.0,
            TOP_RIGHT.1 + coordinate.1,
            CENTRE.0 + coordinate.0 + CENTRE.0 / 3,
            CENTRE.1 + coordinate.1 - CENTRE.1 / 3,
        ));
    }
}

#[cfg(test)]
mod tests {
    use crate::svg_render::Elements;

    use super::super::test_common::*;

    #[test]
    fn test_diagonal_crossing() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::diagonal_crossing(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "diagonal-crossing");
    }

    #[test]
    fn test_crossing_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::crossing_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "crossing-up");
    }

    #[test]
    fn test_crossing_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::crossing_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "crossing-down");
    }

    #[test]
    fn test_tunnel_horizontal() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::tunnel_horizontal(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "tunnel-horizontal");
    }

    #[test]
    fn test_tunnel_vertical() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::tunnel_vertical(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "tunnel-vertical");
    }

    #[test]
    fn test_diagonal_tunnel_up() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::diagonal_tunnel_up(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "diagonal-tunnel-up");
    }

    #[test]
    fn test_diagonal_tunnel_down() {
        let mut svg = String::new();
        svg_prolog(&mut svg);
        Elements::diagonal_tunnel_down(NULL, &mut svg);
        svg_epilog(&mut svg);
        write(svg, "diagonal-tunnel-down");
    }
}
