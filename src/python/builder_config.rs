use std::{fs::File, path::PathBuf};

use crate::schema::{Definition, Extension, Schema, Traversal, Type};

use super::{error::Result, sourcecode::SourceCode};

pub fn render(outdir: &PathBuf, s: &Schema) -> Result<()> {
    let outfile = outdir.join("builder_config.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new_generated();
    let t = Traversal::new(s);

    src.import("from . import impl");
    src.import("from .spec import Spec");

    src.line("class BuilderConfig:");
    src.indent();

    render_directory(&mut src, t)?;
    src.dedent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_directory(src: &mut SourceCode, d: Traversal) -> Result<()> {
    let mut pass = true;
    if d.has_children() {
        for child in d.iter_children() {
            pass = false;

            src.line(format!("class {}:", child.get_module_name()));
            src.indent();
            render_directory(src, child)?;
            src.dedent();
        }
    }
    if d.has_definition() {
        for type_ in d.iter_definitions() {
            match type_ {
                Type::Definition(def) => match def {
                    Definition::ObjectDefinition(inner) => {
                        pass = false;
                        src.line(format!(
                            "{name}: Spec.{module}.{name}Spec = impl.{module}.{name}Impl()",
                            name = &inner.name,
                            module = inner.module.join(".")
                        ));
                    }
                    Definition::InterfaceDefinition(inner) => {
                        pass = false;
                        src.line(format!(
                            "{name}: Spec.{module}.{name}Spec = impl.{module}.{name}Impl()",
                            name = &inner.name,
                            module = inner.module.join(".")
                        ));
                    }
                    Definition::ScalarDefinition(inner) => {
                        pass = false;
                        src.line(format!(
                            "{name}: Spec.{module}.{name}Spec = impl.{module}.{name}Impl()",
                            name = &inner.name,
                            module = inner.module.join(".")
                        ));
                    }
                    _ => {}
                },
                Type::Extension(ext) => match ext {
                    Extension::ObjectExtension(inner) => {
                        pass = false;
                        src.line(format!(
                            "{name}: Spec.{module}.{name}Spec = impl.{module}.{name}Impl()",
                            name = &inner.name,
                            module = inner.module.join(".")
                        ));
                    }
                    Extension::InterfaceExtension(inner) => {
                        pass = false;
                        src.line(format!(
                            "{name}: Spec.{module}.{name}Spec = impl.{module}.{name}Impl()",
                            name = &inner.name,
                            module = inner.module.join(".")
                        ));
                    }
                    _ => {}
                },
            }
            //
        }
    }
    if pass {
        src.line("pass");
    }
    Ok(())
}
