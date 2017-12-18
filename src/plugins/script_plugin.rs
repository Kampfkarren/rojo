use std::collections::HashMap;

use regex::Regex;

use plugin::{Plugin, PluginChain, PluginResult};
use rbx::{RbxItem, RbxValue};
use vfs::VfsItem;

lazy_static! {
    static ref SERVER_PATTERN: Regex = Regex::new(r"^(.*?)\.server\.lua$").unwrap();
    static ref CLIENT_PATTERN: Regex = Regex::new(r"^(.*?)\.client\.lua$").unwrap();
    static ref MODULE_PATTERN: Regex = Regex::new(r"^(.*?)\.lua$").unwrap();
}

static SERVER_INIT: &'static str = "init.server.lua";
static CLIENT_INIT: &'static str = "init.client.lua";
static MODULE_INIT: &'static str = "init.lua";

pub struct ScriptPlugin;

impl ScriptPlugin {
    pub fn new() -> ScriptPlugin {
        ScriptPlugin
    }
}

impl Plugin for ScriptPlugin {
    fn transform_file(&self, plugins: &PluginChain, vfs_item: &VfsItem) -> PluginResult {
        match vfs_item {
            &VfsItem::File { ref contents, ref name } => {
                let (class_name, rbx_name) = {
                    if let Some(captures) = SERVER_PATTERN.captures(name) {
                        ("Script".to_string(), captures.get(1).unwrap().as_str().to_string())
                    } else if let Some(captures) = CLIENT_PATTERN.captures(name) {
                        ("LocalScript".to_string(), captures.get(1).unwrap().as_str().to_string())
                    } else if let Some(captures) = MODULE_PATTERN.captures(name) {
                        ("ModuleScript".to_string(), captures.get(1).unwrap().as_str().to_string())
                    } else {
                        return PluginResult::Pass;
                    }
                };

                let mut properties = HashMap::new();

                properties.insert("Source".to_string(), RbxValue::String {
                    value: contents.clone(),
                });

                PluginResult::Value(Some(RbxItem {
                    name: rbx_name,
                    class_name: class_name,
                    children: Vec::new(),
                    properties,
                }))
            },
            &VfsItem::Dir { ref children, ref name } => {
                let init_item = {
                    let maybe_item = children.get(SERVER_INIT)
                        .or(children.get(CLIENT_INIT))
                        .or(children.get(MODULE_INIT));

                    match maybe_item {
                        Some(v) => v,
                        None => return PluginResult::Pass,
                    }
                };

                let mut rbx_item = match self.transform_file(plugins, init_item) {
                    PluginResult::Value(Some(item)) => item,
                    _ => {
                        eprintln!("Inconsistency detected in ScriptPlugin!");
                        return PluginResult::Pass;
                    },
                };

                rbx_item.name.clear();
                rbx_item.name.push_str(name);

                for (child_name, child_item) in children {
                    if child_name == init_item.name() {
                        continue;
                    }

                    match plugins.transform_file(child_item) {
                        Some(child_rbx_item) => {
                            rbx_item.children.push(child_rbx_item);
                        },
                        _ => {},
                    }
                }

                PluginResult::Value(Some(rbx_item))
            },
        }
    }
}