use std::{fs::File, path::PathBuf};

use crate::schema::{Definition, ModuleRef, Project};

use super::{error::Result, naming, source_code::SourceCode};

pub fn render(outdir: &PathBuf, s: &Project) -> Result<()> {
    let outfile = outdir.join("builder_config.py");

    let mut file = File::create(outfile)?;
    let mut src = SourceCode::new_generated();
    let t = ModuleRef::new(s);

    src.import("from . import runtime");

    src.line("class BuilderConfig:");
    src.indent();

    render_directory(&mut src, t)?;
    src.dedent();

    src.write_to(&mut file)?;

    Ok(())
}

fn render_directory(src: &mut SourceCode, d: ModuleRef) -> Result<()> {
    let mut pass = true;
    if d.has_children() {
        for (i, child) in d.iter_children().enumerate() {
            if i > 0 {
                src.newline();
            }
            pass = false;

            src.line(format!("class {}:", naming::module_name(child.get_name())));
            src.indent();
            render_directory(src, child)?;
            src.dedent();
        }
    }
    if d.has_definition() {
        for type_ in d.iter_definitions() {
            match type_ {
                Definition::ObjectDefinition(_) => {
                    pass = false;
                    render_config_field(src, &d, type_);
                }
                Definition::InterfaceDefinition(_) => {
                    pass = false;
                    render_config_field(src, &d, type_);
                }
                Definition::ScalarDefinition(_) => {
                    pass = false;
                    render_config_field(src, &d, type_);
                }
                Definition::ObjectExtension(_) => {
                    pass = false;
                    render_config_field(src, &d, type_);
                }
                Definition::InterfaceExtension(_) => {
                    pass = false;
                    render_config_field(src, &d, type_);
                }
                _ => {}
            }
            //
        }
    }

    if pass {
        src.line("pass");
    }
    Ok(())
}

fn render_config_field(src: &mut SourceCode, d: &ModuleRef, def: &Definition) {
    let definition_name = def.get_definition_name().unwrap();
    let field_name = naming::build_config_field(&definition_name);
    let module_accessor = naming::module_path(&d.get_breadcrumbs());
    let runtime_name = naming::impl_type(&definition_name);
    src.line(format!(
        "{field_name} = runtime.{module_accessor}.{runtime_name}()"
    ));
}
