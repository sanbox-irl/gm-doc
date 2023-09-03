use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Program {
    /// All the functions in the manual.
    pub functions: Vec<Function>,
    /// All the variables in the manual.
    pub variables: Vec<Variable>,
    /// All the constants in the manual.
    pub constants: Vec<Constant>,
}

/// A function.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Function {
    /// The name of the function
    pub name: String,

    /// The parameters of the function.
    pub parameters: Vec<Parameter>,

    /// The description of what the function does.
    pub description: String,

    /// What the function returns.
    pub returns: String,

    /// Whether the function is deprecated
    pub deprecated: bool,

    /// If the function is pure.
    pub pure: bool,

    /// The link to the webpage.
    pub link: Option<Url>,
}

/// A variable.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Variable {
    /// The name of the variable
    pub name: String,

    /// The description of what the variable does.
    pub description: String,

    /// Whether deprecated.
    pub deprecated: bool,

    /// Whether is readable.
    pub get: bool,

    /// Whether is writable.
    pub set: bool,

    /// Whether is a GmInstance variable.
    pub instance: bool,

    /// The type of the variable.
    pub returns: String,

    /// The link to the webpage.
    pub link: Option<Url>,
}

/// A parameter and description from the manual.
///
/// [`GmManualFunction`]: struct.GmManualFunction.html
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default, Serialize, Deserialize)]
pub struct Parameter {
    /// The name of the parameter.
    pub parameter_name: String,

    /// A description given of the parameter.
    pub description: String,

    /// The GameMaker type of this parameter.
    pub gm_type: String,

    /// Whether this parameter is optional or not.
    pub optional: bool,
}

/// A constant.
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Serialize, Deserialize)]
pub struct Constant {
    /// The name of the constant
    pub name: String,

    /// A description of the constant.
    pub description: String,

    /// The type of the constant.
    pub returns: String,

    /// A "Class" for the constant. Basically a namespace.
    pub class: Option<String>,

    /// Whether the function is deprecated
    pub deprecated: bool,

    /// The link to the webpage.
    pub link: Option<Url>,
}
