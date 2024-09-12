use std::collections::BTreeSet;

pub struct SourceCode {
    prelude: Option<&'static str>,
    indent_unit: String,
    indent_level: i32,
    imports: BTreeSet<String>,
    body: String,
}

impl SourceCode {
    pub fn new() -> SourceCode {
        SourceCode {
            prelude: None,
            indent_unit: "    ".to_string(),
            indent_level: 0,
            imports: BTreeSet::new(),
            body: String::new(),
        }
    }
    pub fn new_generated() -> SourceCode {
        SourceCode {
            prelude: Some("# GENERATED. DO NOT EDIT.\n# fmt: off"),
            indent_unit: "    ".to_string(),
            indent_level: 0,
            imports: BTreeSet::new(),
            body: String::new(),
        }
    }
    pub fn import(&mut self, module: &str) -> String {
        if !self.imports.contains(module) {
            self.imports.insert(module.to_string());
        }
        module.to_string()
    }

    pub fn line<S: std::fmt::Display>(&mut self, line: S) {
        let indent = self.indent_unit.repeat(self.indent_level as usize);
        self.body.push_str(format!("{indent}{line}\n").as_str());
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

    pub fn write_to<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        if let Some(prelude) = self.prelude {
            writeln!(out, "{}", prelude)?;
        }
        for i in &self.imports {
            writeln!(out, "{}", i)?;
        }
        if !self.imports.is_empty() {
            writeln!(out)?;
            writeln!(out)?;
        }
        writeln!(out, "{}", self.body.trim_end())?;
        Ok(())
    }
}
