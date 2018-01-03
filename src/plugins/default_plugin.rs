use std::collections::HashMap;

use core::Route;
use plugin::{Plugin, PluginChain, TransformFileResult, RbxChangeResult, FileChangeResult};
use rbx::{RbxItem, RbxValue};
use vfs::VfsItem;

/// A plugin with simple transforms:
/// * Directories become Folder instances
/// * Files become StringValue objects with 'Value' as their contents
pub struct DefaultPlugin;

impl DefaultPlugin {
    pub fn new() -> DefaultPlugin {
        DefaultPlugin
    }
}

impl Plugin for DefaultPlugin {
    fn transform_file(&self, plugins: &PluginChain, route: &Route, vfs_item: &VfsItem) -> TransformFileResult {
        match vfs_item {
            &VfsItem::File { ref contents, ref name } => {
                let mut properties = HashMap::new();

                properties.insert("Value".to_string(), RbxValue::String {
                    value: contents.clone(),
                });

                TransformFileResult::Value(Some(RbxItem {
                    name: name.clone(),
                    class_name: "StringValue".to_string(),
                    children: Vec::new(),
                    properties,
                    route: Some(route.clone()),
                }))
            },
            &VfsItem::Dir { ref children, ref name } => {
                let mut rbx_children = Vec::new();

                for (child_name, child_item) in children {
                    let mut child_route = Vec::new();
                    child_route.extend_from_slice(route);
                    child_route.push(child_name.clone());

                    match plugins.transform_file(&child_route, child_item) {
                        Some(rbx_item) => {
                            rbx_children.push(rbx_item);
                        },
                        _ => {},
                    }
                }

                TransformFileResult::Value(Some(RbxItem {
                    name: name.clone(),
                    class_name: "Folder".to_string(),
                    children: rbx_children,
                    properties: HashMap::new(),
                    route: Some(route.clone()),
                }))
            },
        }
    }

    fn handle_file_change(&self, route: &Route) -> FileChangeResult {
        FileChangeResult::MarkChanged(Some(vec![route.clone()]))
    }

    fn handle_rbx_change(&self, _route: &Route, _rbx_item: &RbxItem) -> RbxChangeResult {
        RbxChangeResult::Pass
    }
}
