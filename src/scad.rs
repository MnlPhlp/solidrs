use crate::element::{Element, InnerElement};
use std::fmt::Write;

pub trait ExportScad: Sized {
    fn render_scad(self) -> String;

    fn save_as_scad(self, name: &str) {
        let name = if name.ends_with(".scad") {
            String::from(name)
        } else {
            format!("{name}.scad")
        };
        let rendered = self.render_scad();
        std::fs::write(name, rendered).unwrap()
    }
}

impl ExportScad for &Element {
    fn render_scad(self) -> String {
        let mut w = Writer::new();
        w.render(&self.0);
        w.str
    }
}

impl<'a, COLLECTION: IntoIterator<Item = &'a Element>> ExportScad for COLLECTION {
    fn render_scad(self) -> String {
        let mut w = Writer::new();
        for element in self {
            w.render(&element.0);
        }
        w.str
    }
}

macro_rules! render {
    ($r:expr, $($arg:tt)*) => {
        {
            write!($r.str,"{}","    ".repeat($r.indent)).unwrap();
            write!($r.str,$($arg)*).unwrap();
        }
    };
}
macro_rules! renderln {
    ($r:expr, $($arg:tt)*) => {
        {
            write!($r.str,"{}","    ".repeat($r.indent)).unwrap();
            writeln!($r.str,$($arg)*).unwrap();
        }
    };
}
macro_rules! renderln_no_indent {
    ($r:expr, $($arg:tt)*) => {
        {
            writeln!($r.str,$($arg)*).unwrap();
        }
    };
}

struct Writer {
    str: String,
    indent: usize,
}

impl Writer {
    fn new() -> Self {
        Self {
            str: String::new(),
            indent: 0,
        }
    }

    fn render(&mut self, root: &InnerElement) {
        match root {
            InnerElement::Empty => {}
            InnerElement::Cube { x, y, z, centered } => self.render_cube(*x, *y, *z, *centered),
            InnerElement::Cylinder { h, r, centered } => self.render_cylinder(*r, *h, *centered),
            InnerElement::Square { x, y, centered } => self.render_square(*x, *y, *centered),
            InnerElement::Union { children } => self.render_union(children),
            InnerElement::Diff { children } => self.render_diff(children),
            InnerElement::Translate { x, y, z, child } => {
                self.render_transform("translate", *x, *y, *z, child)
            }
            InnerElement::Rotate { x, y, z, child } => {
                self.render_transform("rotate", *x, *y, *z, child)
            }
            InnerElement::RotateExtrude { angle, child } => self.render_rot_ext(*angle, child),
        }
    }

    fn render_cube(&mut self, x: f32, y: f32, z: f32, centered: bool) {
        renderln!(self, "cube([{x},{y},{z}]{});", center(centered));
    }

    fn render_transform(&mut self, transform: &str, x: f32, y: f32, z: f32, child: &InnerElement) {
        render!(self, "{transform}([{x},{y},{z}])");
        self.render_child(child);
    }

    fn render_diff(&mut self, children: &[InnerElement]) {
        render!(self, "difference()");
        self.render_children(children);
    }

    fn render_union(&mut self, children: &[InnerElement]) {
        render!(self, "union()");
        self.render_children(children);
    }

    fn render_children(&mut self, children: &[InnerElement]) {
        renderln_no_indent!(self, "{{");
        self.indent += 1;
        for child in children {
            self.render(child)
        }
        self.indent -= 1;
        renderln!(self, "}}");
    }

    fn render_child(&mut self, child: &InnerElement) {
        renderln_no_indent!(self, "{{");
        self.indent += 1;
        self.render(child);
        self.indent -= 1;
        renderln!(self, "}}");
    }

    fn render_square(&mut self, x: f32, y: f32, centered: bool) {
        renderln!(self, "square([{x},{y}]{});", center(centered));
    }

    fn render_rot_ext(&mut self, angle: f32, child: &InnerElement) {
        render!(self, "rotate_extrude(angle={angle})");
        self.render_child(child);
    }

    fn render_cylinder(&mut self, r: f32, h: f32, centered: bool) {
        renderln!(self, "cylinder({h}, r = {r}{});", center(centered))
    }
}

fn center(c: bool) -> &'static str {
    if c {
        ",center = true"
    } else {
        ""
    }
}
