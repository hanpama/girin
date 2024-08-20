use std::collections::BTreeSet;

pub struct SourceCode {
    prelude: Option<&'static str>,
    indent_unit: String,
    indent_level: i32,
    imports: BTreeSet<String>,
    body: Vec<String>,
    list_index: i32,
}

struct Import {
    from: String,
    import: String,
}

impl Import {
    fn from_expr(expr: &str) -> Self {
        let import: String;
        let from: String;

        if expr.contains(".") {
            let parts: Vec<&str> = expr.split('.').collect();
            import = parts[parts.len() - 1].to_owned();
            from = match parts.len() {
                1 => "".to_owned(),
                2 => format!(".{}", parts[0]),
                i => parts[..i - 1]
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<String>>()
                    .join("."),
            }
        } else {
            from = "".to_owned();
            import = expr.to_owned();
        }

        Import { from, import }
    }

    fn to_string(&self) -> String {
        if self.from.is_empty() {
            format!("import {}", self.import)
        } else {
            format!("from {} import {}", self.from, self.import)
        }
    }
}

impl SourceCode {
    pub fn new() -> SourceCode {
        SourceCode {
            prelude: None,
            indent_unit: "    ".to_string(),
            indent_level: 0,
            imports: BTreeSet::new(),
            body: Vec::new(),
            list_index: 0,
        }
    }
    pub fn new_generated() -> SourceCode {
        SourceCode {
            prelude: Some("# GENERATED. DO NOT EDIT.\n# fmt: off"),
            indent_unit: "    ".to_string(),
            indent_level: 0,
            imports: BTreeSet::new(),
            body: Vec::new(),
            list_index: 0,
        }
    }
    pub fn import(&mut self, module: &str) -> String {
        if !self.imports.contains(module) {
            self.imports.insert(module.to_string());
        }
        module.to_string()
    }

    pub fn line<S: std::fmt::Display>(&mut self, line: S) -> &mut Self {
        self.body.push(format!(
            "{}{}{}",
            self.indent_unit.repeat(self.indent_level as usize),
            line,
            if self.list_index > 0 { "," } else { "" }
        ));
        self
    }

    pub fn indent(&mut self) -> &mut Self {
        self.indent_level += 1;
        self
    }
    pub fn dedent(&mut self) -> &mut Self {
        self.indent_level -= 1;
        self
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
            writeln!(out)?;
        }
        for line in &self.body {
            writeln!(out, "{}", line)?;
        }
        Ok(())
    }
}
