use nom::Parser;
use ::value::Value;
use value::Value::Integer;
use vrl::prelude::*;

fn seahash(value: Value) -> Resolved {
    let value = value.try_bytes()?;
    Ok(Integer(seahash::hash(&value) as i64))
}

#[derive(Clone, Copy, Debug)]
pub struct Seahash;

impl Function for Seahash {
    fn identifier(&self) -> &'static str {
        "seahash"
    }

    fn examples(&self) -> &'static [Example] {
        &[
            Example {
                title: "seahash",
                source: r#"seahash("foobar")"#,
                result: Ok("5348458858952426560"),
            },
            Example {
                title: "seahash above i64.MAX",
                source: r#"seahash("bar")"#,
                result: Ok("-2796170501982571315"),
            }
        ]
    }

    fn compile(
        &self,
        _state: &state::TypeState,
        _ctx: &mut FunctionCompileContext,
        arguments: ArgumentList,
    ) -> Compiled {
        let value = arguments.required("value");

        Ok(SeahashFn { value }.as_expr())
    }

    fn parameters(&self) -> &'static [Parameter] {
        &[Parameter {
            keyword: "value",
            kind: kind::BYTES,
            required: true,
        }]
    }
}

#[derive(Debug, Clone)]
struct SeahashFn {
    value: Box<dyn Expression>,
}

impl FunctionExpression for SeahashFn {
    fn resolve(&self, ctx: &mut Context) -> Resolved {
        let value = self.value.resolve(ctx)?;
        seahash(value)
    }

    fn type_def(&self, _: &state::TypeState) -> TypeDef {
        TypeDef::bytes().infallible()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    test_function![
        seahash => Seahash;

        seahash {
             args: func_args![value: "foo"],
             want: Ok("4413582353838009230"),
             tdef: TypeDef::bytes().infallible(),
        }

        seahash_buffer_overflow {
             args: func_args![value: "bar"],
             want: Ok("-2796170501982571315"),
             tdef: TypeDef::bytes().infallible(),
        }
    ];
}
