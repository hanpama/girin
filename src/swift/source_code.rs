use std::collections::BTreeSet;

pub struct SourceCode {
    prelude: String,
    indent_unit: String,
    indent_level: i32,
    imports: BTreeSet<String>,
    body: String,
}

impl SourceCode {
    pub fn new() -> SourceCode {
        SourceCode {
            prelude: String::new(),
            indent_unit: "    ".to_string(),
            indent_level: 0,
            imports: BTreeSet::new(),
            body: String::new(),
        }
    }

    pub fn import(&mut self, expr: &str) {
        self.imports.insert(expr.to_owned());
    }
    pub fn append<S: std::fmt::Display>(&mut self, text: S) {
        self.body.push_str(&format!("{}", text));
    }
    pub fn line<S: std::fmt::Display>(&mut self, line: S) {
        let indent = self.indent_unit.repeat(self.indent_level as usize);
        self.body.push_str(&format!("\n{indent}{line}",));
    }
    pub fn newline(&mut self) {
        self.body.push_str("\n");
    }

    pub fn indent(&mut self) {
        self.indent_level += 1;
    }

    pub fn dedent(&mut self) {
        self.indent_level -= 1;
    }

    pub fn write_to<W: std::io::Write>(&self, w: &mut W) -> std::io::Result<()> {
        for import in &self.imports {
            writeln!(w, "import {}", import)?;
        }
        if self.imports.len() > 0 {
            writeln!(w)?;
        }
        write!(w, "{}\n", self.body.trim())
    }
}
