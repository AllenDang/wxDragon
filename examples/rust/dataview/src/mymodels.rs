use crate::music_tree::{MusicNode, MusicTree, NodeType};
use wxdragon::prelude::*;

pub fn create_music_tree_model(data: MusicTree) -> CustomDataViewTreeModel {
    CustomDataViewTreeModel::new(
        data,
        move |data: &MusicTree, item: Option<&MusicNode>| match item {
            None => None,
            Some(node) => data
                .parent_of(node)
                .map(|rc| &*rc.borrow() as *const MusicNode as *mut MusicNode),
        },
        move |_data, item: Option<&MusicNode>| match item {
            None => true,
            Some(node) => matches!(node.node_type, NodeType::Branch),
        },
        move |data: &MusicTree, item: Option<&MusicNode>| match item {
            None => vec![&*data.root.borrow() as *const MusicNode as *mut MusicNode],
            Some(node) => node
                .children
                .as_ref()
                .into_iter()
                .flat_map(|v| v.iter())
                .map(|b| &*b.borrow() as *const MusicNode as *mut MusicNode)
                .collect(),
        },
        move |data: &MusicTree, item: Option<&MusicNode>, col| match item {
            None => {
                let r_b = data.root.borrow();
                let r = &*r_b;
                match col {
                    0 => Variant::String(r.title.clone()),
                    1 => Variant::String(r.artist.clone().unwrap_or_default()),
                    2 => match r.year {
                        Some(v) => Variant::String(v.to_string()),
                        None => Variant::String(String::new()),
                    },
                    3 => Variant::String(r.quality.clone().unwrap_or_default()),
                    _ => Variant::String(String::new()),
                }
            }
            Some(n) => match col {
                0 => Variant::String(n.title.clone()),
                1 => Variant::String(n.artist.clone().unwrap_or_default()),
                2 => match n.year {
                    Some(v) => Variant::String(v.to_string()),
                    None => Variant::String(String::new()),
                },
                3 => Variant::String(n.quality.clone().unwrap_or_default()),
                _ => Variant::String(String::new()),
            },
        },
        Some(move |_: &MusicTree, _: Option<&MusicNode>, _: u32, _: &Variant| false),
        Some(move |_: &MusicTree, _: Option<&MusicNode>, _: u32| true),
        Some(
            move |_: &MusicTree, a: &MusicNode, b: &MusicNode, col: u32, asc: bool| {
                if col == 2 {
                    let va = a.year.unwrap_or(0);
                    let vb = b.year.unwrap_or(0);
                    // return negative if a < b, positive if a > b
                    let diff = va.wrapping_sub(vb);
                    if asc {
                        diff
                    } else {
                        -diff
                    }
                } else {
                    0
                }
            },
        ),
    )
}
