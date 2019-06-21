use i3ipc::reply::{Node, NodeType, Workspace};
use i3ipc::I3Connection;

fn find_active_workspace(workspaces: Vec<Workspace>) -> Workspace {
    for workspace in workspaces {
        if workspace.focused {
            return workspace;
        }
    }
    panic!("no focused workspace");
}

fn find_active_workspace_in_tree(root: Node, active_ws: &Workspace) -> Option<Vec<Node>> {
    match root.nodetype {
        NodeType::Workspace => {
            let root_name = root.name.unwrap();
            if root_name == active_ws.name {
                Some(root.nodes)
            } else {
                None
            }
        }
        _ => {
            for node in root.nodes {
                match find_active_workspace_in_tree(node, &active_ws) {
                    Some(n) => {
                        return Some(n);
                    }
                    None => {}
                }
            }
            None
        }
    }
}

#[derive(Debug)]
struct NodePtr {
    id: i64,
    focused: bool,
}

fn collect_nodes(root: Node, coll: &mut Vec<NodePtr>) {
    if root.window.is_some() {
        coll.push(NodePtr {
            id: root.id,
            focused: root.focused,
        });
    }
    for node in root.nodes {
        collect_nodes(node, coll);
    }
}

fn main() {
    let mut i3 = I3Connection::connect().unwrap();
    let workspaces = i3.get_workspaces().unwrap();
    let workspace = find_active_workspace(workspaces.workspaces);
    let root = i3.get_tree().unwrap();
    let ws_node = find_active_workspace_in_tree(root, &workspace).unwrap();
    let mut ws_nodes = Vec::new();
    for node in ws_node {
        collect_nodes(node, &mut ws_nodes);
    }

    let mut focused_i = None;
    for (i, node) in ws_nodes.iter().enumerate() {
        if node.focused {
            focused_i = Some(i);
            break;
        }
    }
    let next_i = ((focused_i.unwrap() + 1) % ws_nodes.len());
    i3.run_command(&format!("[con_id={}] focus", ws_nodes[next_i].id))
        .unwrap();
}
