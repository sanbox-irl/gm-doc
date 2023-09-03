use gm_doc::*;
use std::collections::HashMap;
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

    let mut functions = HashMap::new();
    let mut variables = HashMap::new();
    let mut constants = HashMap::new();

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
                        let name = sub_field.attribute("Name").unwrap().to_owned();
                        let gm_type = sub_field.attribute("Type").unwrap().to_owned();
                        let optional: bool =
                            sub_field.attribute("Optional").unwrap().parse().unwrap();
                        let description = sub_field.text().unwrap_or_default().to_owned();

                        parameters.push(Parameter {
                            name: name.to_string(),
                            description,
                            gm_type,
                            optional,
                        });
                    }
                }

                let link = make_link(&name);

                functions.insert(
                    name.clone(),
                    Function {
                        name,
                        parameters,
                        description: description.unwrap_or_default().to_owned(),
                        deprecated,
                        returns,
                        pure,
                        link,
                    },
                );
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

                variables.insert(
                    name.clone(),
                    Variable {
                        name,
                        description,
                        returns: gm_type,
                        link,
                        deprecated,
                        get,
                        set,
                        instance,
                    },
                );
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

                constants.insert(
                    name.clone(),
                    Constant {
                        name,
                        description,
                        class,
                        deprecated,
                        link,
                        returns: gm_type,
                    },
                );
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
