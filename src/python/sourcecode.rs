use std::collections::BTreeMap;

pub struct SourceCode {
    prelude: String,
    indent_unit: String,
    indent_level: i32,
    std_imports: BTreeMap<String, Import>,
    first_imports: BTreeMap<String, Import>,
    third_imports: BTreeMap<String, Import>,
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
            prelude: "# GENERATED. DO NOT EDIT.\n# fmt: off".to_string(),
            indent_unit: "    ".to_string(),
            indent_level: 0,
            std_imports: BTreeMap::new(),
            first_imports: BTreeMap::new(),
            third_imports: BTreeMap::new(),
            body: Vec::new(),
            list_index: 0,
        }
    }

    pub fn import_std(&mut self, module: &str) -> String {
        if !self.std_imports.contains_key(module) {
            self.std_imports
                .insert(module.to_string(), Import::from_expr(module));
        }
        self.std_imports.get(module).unwrap().import.clone()
    }

    pub fn import_first(&mut self, module: &str) -> String {
        if !self.first_imports.contains_key(module) {
            self.first_imports
                .insert(module.to_string(), Import::from_expr(module));
        }
        self.first_imports.get(module).unwrap().import.clone()
    }

    pub fn import_third(&mut self, expr: &str) -> String {
        if !self.third_imports.contains_key(expr) {
            self.third_imports
                .insert(expr.to_string(), Import::from_expr(expr));
        }
        self.third_imports.get(expr).unwrap().import.clone()
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
        writeln!(out, "{}", self.prelude)?;
        for i in self.std_imports.values() {
            writeln!(out, "{}", i.to_string())?;
        }
        if !self.std_imports.is_empty() {
            writeln!(out)?;
        }
        for i in self.third_imports.values() {
            writeln!(out, "{}", i.to_string())?;
        }
        if !self.third_imports.is_empty() {
            writeln!(out)?;
        }
        for i in self.first_imports.values() {
            writeln!(out, "{}", i.to_string())?;
        }
        if !self.first_imports.is_empty() {
            writeln!(out)?;
        }
        if !self.std_imports.is_empty()
            || !self.third_imports.is_empty()
            || !self.first_imports.is_empty()
        {
            writeln!(out)?;
        }
        for line in &self.body {
            writeln!(out, "{}", line)?;
        }
        Ok(())
    }
}
