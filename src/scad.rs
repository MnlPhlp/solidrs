use crate::{
    element::{Element, InnerElement},
    var::{Val, Var},
};
use std::{collections::HashMap, fmt::Write, path::Path};

pub trait Export: Sized {
    fn render_scad(self) -> String;

    fn save_as_scad(self, name: &str) {
        let is_scad = Path::new(name)
            .extension()
            .map_or(false, |ext| ext.eq_ignore_ascii_case("scad"));
        let name = if is_scad {
            String::from(name)
        } else {
            format!("{name}.scad")
        };
        let rendered = self.render_scad();
        std::fs::write(name, rendered).unwrap();
    }
}

impl Export for &Element<'_> {
    fn render_scad(self) -> String {
        let mut w = Writer::new();
        w.render_vars(&self.0);
        w.render(&self.0);
        w.str
    }
}

impl<'a, COLLECTION: IntoIterator<Item = &'a Element<'a>>> Export for COLLECTION {
    fn render_scad(self) -> String {
        let mut w = Writer::new();
        for element in self {
            w.render_vars(&element.0);
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
            InnerElement::Cube { x, y, z, centered } => self.render_cube(x, y, z, *centered),
            InnerElement::Cylinder { h, r, centered } => self.render_cylinder(r, h, *centered),
            InnerElement::Square { x, y, centered } => self.render_square(x, y, *centered),
            InnerElement::Union { children } => self.render_union(children),
            InnerElement::Diff { children } => self.render_diff(children),
            InnerElement::Translate { x, y, z, child } => {
                self.render_transform("translate", x, y, z, child);
            }
            InnerElement::Rotate { x, y, z, child } => {
                self.render_transform("rotate", x, y, z, child);
            }
            InnerElement::RotateExtrude { angle, child } => self.render_rot_ext(angle, child),
            InnerElement::Fa { fa, child } => self.render_config_param("fa", fa, child),
            InnerElement::Fs { fs, child } => self.render_config_param("fs", fs, child),
        }
    }

    fn render_cube(&mut self, x: &Val, y: &Val, z: &Val, centered: bool) {
        renderln!(self, "cube([{x},{y},{z}]{});", center(centered));
    }

    fn render_transform(
        &mut self,
        transform: &str,
        x: &Val,
        y: &Val,
        z: &Val,
        child: &InnerElement,
    ) {
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
            self.render(child);
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

    fn render_square(&mut self, x: &Val, y: &Val, centered: bool) {
        renderln!(self, "square([{x},{y}]{});", center(centered));
    }

    fn render_rot_ext(&mut self, angle: &Val, child: &InnerElement) {
        render!(self, "rotate_extrude(angle={angle})");
        self.render_child(child);
    }

    fn render_cylinder(&mut self, r: &Val, h: &Val, centered: bool) {
        renderln!(self, "cylinder({h}, r = {r}{});", center(centered));
    }

    fn render_config_param(&mut self, name: &str, val: &Val<'_>, child: &InnerElement<'_>) {
        renderln!(self, "${name} = {val};");
        self.render(child);
    }

    fn render_vars(&mut self, element: &InnerElement) {
        let mut vars = HashMap::new();
        collect_vars(&mut vars, element);
        let mut vars = vars.values().collect::<Vec<_>>();
        vars.sort_unstable_by(|a, b| a.get_name().cmp(b.get_name()));
        for var in vars {
            if !var.get_comment().is_empty() {
                renderln!(self, "// {}", var.get_comment());
            }
            renderln!(self, "{} = {};", var.get_name(), var.get_val());
        }
    }
}

fn collect_vars<'a>(map: &mut HashMap<&str, &'a Var>, element: &'a InnerElement) {
    match element {
        InnerElement::Empty => {}
        InnerElement::Cube { x, y, z, .. } => add_vars(map, &[x, y, z]),
        InnerElement::Cylinder { h, r, .. } => add_vars(map, &[h, r]),
        InnerElement::Square { x, y, .. } => add_vars(map, &[x, y]),
        InnerElement::Union { children } | InnerElement::Diff { children } => {
            children.iter().for_each(|c| collect_vars(map, c));
        }
        InnerElement::Translate { x, y, z, child } | InnerElement::Rotate { x, y, z, child } => {
            add_vars(map, &[x, y, z]);
            collect_vars(map, child);
        }
        InnerElement::RotateExtrude { angle: val, child }
        | InnerElement::Fs { fs: val, child }
        | InnerElement::Fa { fa: val, child } => {
            add_vars(map, &[val]);
            collect_vars(map, child);
        }
    }
}

fn add_vars<'a>(map: &mut HashMap<&str, &'a Var>, vars: &[&'a Val<'_>]) {
    for var in vars {
        if let Val::Var(var) = var {
            let name = var.get_name();
            if !name.is_empty() && !map.contains_key(name) {
                map.insert(name, *var);
            }
        }
    }
}

fn center(c: bool) -> &'static str {
    if c {
        ",center = true"
    } else {
        ""
    }
}
