use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use url::Url;

fn main() {
    let txt = include_str!("../GmlSpec.xml");
    let xml = roxmltree::Document::parse(txt).unwrap();
    let help_docs: HashMap<String, String> =
        serde_json::from_str(include_str!("../helpdocs_keywords.json")).unwrap();
    let make_link = |name: &str| -> Option<Url> {
        help_docs.get(name).map(|extension| {
            Url::parse(&format!("https://manual.yoyogames.com/{}.htm", extension)).unwrap()
        })
    };

    let mut functions = vec![];
    let mut variables = vec![];
    let mut constants = vec![];

    for top_level in xml.descendants() {
        match top_level.tag_name().name() {
            "Function" => {
                let name = top_level.attribute("Name").unwrap().to_string();
                let deprecated: bool = top_level.attribute("Deprecated").unwrap().parse().unwrap();
                let pure: bool = top_level.attribute("Pure").unwrap().parse().unwrap();
                let returns = top_level.attribute("ReturnType").unwrap().to_owned();

                let mut description = None;
                let mut parameters: Vec<Parameter> = vec![];

                for sub_field in top_level.descendants() {
                    if sub_field.has_tag_name("Description") {
                        let txt = sub_field.text().unwrap();
                        description = Some(txt);
                    }

                    if sub_field.has_tag_name("Parameter") {
                        let parameter_name = sub_field.attribute("Name").unwrap().to_owned();
                        let gm_type = sub_field.attribute("Type").unwrap().to_owned();
                        let optional: bool =
                            sub_field.attribute("Optional").unwrap().parse().unwrap();
                        let description = sub_field.text().unwrap_or_default().to_owned();

                        parameters.push(Parameter {
                            parameter_name: parameter_name.to_string(),
                            description,
                            gm_type,
                            optional,
                        });
                    }
                }

                let link = make_link(&name);

                functions.push(Function {
                    name,
                    parameters,
                    description: description.unwrap_or_default().to_owned(),
                    deprecated,
                    returns,
                    pure,
                    link,
                });
            }

            "Variable" => {
                let name = top_level.attribute("Name").unwrap().to_string();
                let gm_type = top_level.attribute("Type").unwrap().to_owned();
                let deprecated: bool = top_level.attribute("Deprecated").unwrap().parse().unwrap();
                let get: bool = top_level.attribute("Get").unwrap().parse().unwrap();
                let set: bool = top_level.attribute("Set").unwrap().parse().unwrap();
                let instance: bool = top_level.attribute("Instance").unwrap().parse().unwrap();
                let description = top_level.text().unwrap_or_default().to_owned();

                let link = make_link(&name);

                variables.push(Variable {
                    name,
                    description,
                    returns: gm_type,
                    link,
                    deprecated,
                    get,
                    set,
                    instance,
                });
            }
            "Constant" => {
                let name = top_level.attribute("Name").unwrap().to_string();

                // who know what's up with this!
                if name == "$$implicit_argument$$" {
                    continue;
                }

                let gm_type = top_level.attribute("Type").unwrap().to_owned();
                let deprecated: bool = top_level.attribute("Deprecated").unwrap().parse().unwrap();
                let class = top_level.attribute("Class").map(|v| v.to_owned());
                let description = top_level.text().unwrap_or_default().to_owned();

                let link = make_link(&name);

                constants.push(Constant {
                    name,
                    description,
                    class,
                    deprecated,
                    link,
                    returns: gm_type,
                })
            }
            "Structures" | "Enumerations" => {
                // we're going to ignore these for now!
            }

            // ignore for now
            _ => {}
        }
    }

    let program = Program {
        functions,
        variables,
        constants,
    };

    println!("{}", serde_json::to_string_pretty(&program).unwrap());
}

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
