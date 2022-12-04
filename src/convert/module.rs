use Werilog::prelude::*;

use super::io::io;

pub fn module(ast: ModuleDeclaration) -> String {
    //TODO:attribute
    match ast {
        ModuleDeclaration::Ports(_, name, _, _, _) => {
            format!(r#"
class {} extends Component {{
  {}
}}

"#,
                name,
                "todo"//TODO
            )
        },
        ModuleDeclaration::NonPorts(_, name, _, port, _) => {
            format!(r#"
class {} extends Component {{
{}
  noIoPrefix()

}}

"#,
                name,
                port.map(io).unwrap_or_else(|| "".to_string())
            )
        },
    }
}
