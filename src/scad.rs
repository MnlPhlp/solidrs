use crate::element::{Element, InnerElement};
use std::{borrow::Borrow, fmt::Write};

impl Element {
    pub fn save_as_scad(&self, name: &str) {
        let name = if name.ends_with(".scad") {
            String::from(name)
        } else {
            format!("{name}.scad")
        };
        let rendered = self.render_scad();
        std::fs::write(name, rendered).unwrap()
    }

    pub fn render_scad(&self) -> String {
        let mut w = Writer::new();
        w.render(&self.0);
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
            InnerElement::Cube { x, y, z, centered } => self.render_cube(*x, *y, *z, centered),
            InnerElement::Union { children } => self.render_union(children),
            InnerElement::Diff { children } => self.render_diff(children),
            InnerElement::Translate { x, y, z, child } => self.render_translate(*x, *y, *z, child),
        }
    }

    fn render_cube(&mut self, x: u32, y: u32, z: u32, centered: &bool) {
        renderln!(self, "cube([{x},{y},{z}],center = {centered});",);
    }

    fn render_translate(&mut self, x: u32, y: u32, z: u32, child: &InnerElement) {
        render!(self, "translate([{x},{y},{z}])");
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
}
