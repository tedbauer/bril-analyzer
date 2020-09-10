use serde::{Deserialize, Serialize};
use serde_json::Result;


#[serde(untagged)]
#[derive(Serialize, Deserialize)]
pub enum LiteralValue {
    NumberValue(i32),
    BooleanValue(bool)
}

// TODO: Parameterized type
#[serde(untagged)]
#[derive(Serialize, Deserialize)]
pub enum Type {
    Primitive(String),
}

#[serde(untagged)]
#[derive(Serialize, Deserialize)]
pub enum Instruction {
    Label { 
        label: String
    },

    Constant {
        op: String,
        dest: String,
        
        #[serde(rename="type")]
        type_: Type,

        value: LiteralValue,
    },

    ValueOperation {
        op: String,
        dest: String,

        #[serde(rename="type")]
        type_: Type,

        args: Option<Vec<String>>,
        funcs: Option<Vec<String>>,
        labels: Option<Vec<String>>
    },

    EffectOperation {
        op: String,
        args: Option<Vec<String>>,
        funcs: Option<Vec<String>>,
        labels: Option<Vec<String>>
    }
}

#[derive(Serialize, Deserialize)]
pub struct Function {
    pub name: String,
    pub instrs: Vec<Instruction>
}

#[derive(Serialize, Deserialize)]
pub struct Program {
    pub functions: Vec<Function>,
}
