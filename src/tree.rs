use std::collections::VecDeque;
use std::fmt::Debug;

use i3_ipc::reply::{Node, NodeType};

use crate::window::{NodeWindowExtractor, Window};

#[derive(Debug)]
pub struct Tree<'a> {
    pub workspaces: Vec<Workspace<'a>>,
}

#[derive(Debug)]
pub struct Workspace<'a> {
    pub num: i32,
    pub name: &'a String,
    pub output: &'a String,
    pub windows: Vec<Window<'a>>,
}

pub trait WorkspaceExtractor {
    fn extract_workspace(&self) -> Option<Workspace>;
}

impl<'a> Tree<'a> {
    pub fn new(root_node: &'a Node) -> Self {
        Self {
            workspaces: Self::extract_workspaces(root_node),
        }
    }

    fn extract_workspaces(root_node: &'a Node) -> Vec<Workspace> {
        let mut queue = VecDeque::new();
        queue.extend(&root_node.nodes);

        let mut workspaces = vec![];

        while let Some(node) = queue.pop_front() {
            if node.node_type == NodeType::Workspace {
                if let Some(workspace) = node.extract_workspace() {
                    workspaces.push(workspace);
                }
            } else {
                queue.extend(&node.nodes);
            }
        }

        workspaces
    }

    pub fn find_workspace_for_window(&self, window_id: usize) -> Option<&'a Workspace> {
        self.workspaces
            .iter()
            .find(|w| w.windows.iter().any(|w| w.id == window_id))
    }

    pub fn find_workspace(&self, workspace_num: i32) -> Option<&'a Workspace> {
        self.workspaces.iter().find(|w| w.num == workspace_num)
    }
}

impl WorkspaceExtractor for Node {
    fn extract_workspace(&self) -> Option<Workspace> {
        let workspace = Workspace {
            num: self.num.expect("Workspace without number"),
            name: self.name.as_ref().expect("Workspace without name"),
            output: self.output.as_ref().expect("Workspace without output"),
            windows: self.extract_windows(),
        };

        if workspace.output != "__i3" {
            Some(workspace)
        } else {
            None
        }
    }
}
