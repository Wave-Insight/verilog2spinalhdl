use Werilog::prelude::*;

pub fn io(ast: Vec<PortDeclaration>) -> String {
    //TODO: attribute
    //TODO: NetType
    let signals = ast.into_iter().map(|x| match x {
        PortDeclaration::Inout(_, (_, signed, range, name)) => {
            format!("    val {} = inout {}\n",
                name.into_iter().reduce(|a,b| a + ", " + &b).unwrap(),
                type_range(signed, range)
            )
        },
        PortDeclaration::Input(_, (_, signed, range, name)) => {
            format!("    val {} = in {}\n",
                name.into_iter().reduce(|a,b| a + ", " + &b).unwrap(),
                type_range(signed, range)
            )
        },
        PortDeclaration::Output(_, s) => {
            let name;
            let wire_or_reg;
            let type_and_range;
            match s {
                OutputDeclaration::Wire((((_, signed), range), n)) => {
                    name = n.into_iter().reduce(|a,b| a + "," + &b).unwrap();
                    wire_or_reg = "".to_string();
                    type_and_range = type_range(signed, range);
                },
                OutputDeclaration::Reg(((signed, range), n)) => {
                    name = n.into_iter()
                        .map(|x| x.0)
                        .reduce(|a,b| a + "," + &b).unwrap();
                    wire_or_reg = "reg ".to_string();
                    type_and_range = type_range(signed, range);
                },
                OutputDeclaration::Others((_, n)) => {
                    name = n.into_iter()
                        .map(|x| x.0)
                        .reduce(|a,b| a + "," + &b).unwrap();
                    wire_or_reg = "todo".to_string();
                    type_and_range = "".to_string();
                },
            };
            format!("    val {} = out {}\n",
                name,
                wire_or_reg + &type_and_range
            )
        },
    }).reduce(|a,b| a+&b).unwrap_or_default();
    format!("  val io = new Bundle {{\n{}  }}\n", signals)
}

fn type_range(signed: bool, range: Option<Range>) -> String {
    if signed {
        range.map(|r| format!("SInt({} bits)", r.to_string())).unwrap_or_else(|| "error?".to_string())
    } else if let Some(r) = range {
        format!("Bits({} bits)", r.to_string())
    } else {
        "Bool()".to_string()
    }
}

impl ToString for Range {
    fn to_string(&self) -> String {
        format!("{} - {} + 1", self.0.to_string(), self.1.to_string())
    }
}

impl ToString for ConstantExpression {
    fn to_string(&self) -> String {
        match self {
            ConstantExpression::ConstantPrimary(c) => c.to_string(),
            ConstantExpression::Unary(_, _, _) => "todo".to_string(),
            ConstantExpression::Binary(l, op, _, r) => {
                format!("{}{}{}", l.to_string(), op, r.to_string())
            },
            ConstantExpression::Condition(_, _, _, _) => "todo".to_string(),
        }
    }
}

impl ToString for ConstantPrimary {
    fn to_string(&self) -> String {
        match self {
            ConstantPrimary::Number(n) => n.to_string(),
            ConstantPrimary::Parameter(p) => p.to_string(),
        }
    }
}

impl ToString for Number {
    fn to_string(&self) -> String {
        match self {
            Number::Decimal(s) => s.to_string(),
            Number::Octal(s) => s.to_string(),
            Number::Binary(s) => s.to_string(),
            Number::Hex(s) => s.to_string(),
            Number::Real(s) => s.to_string(),
        }
    }
}

trait ToString {
    fn to_string(&self) -> String;
}
