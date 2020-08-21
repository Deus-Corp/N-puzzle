use std::collections::BinaryHeap;
use super::puzzle::Puzzle;

pub fn reverse_path<T>(goal: Puzzle) -> Vec<Puzzle>{
    unimplemented!();
    // let mut current = goal;
    // let path = vec![];
    // while let Some(parent) = current.parent {
    //     path.push(*current);
    //     current = &parent;
    // }
    // path
}

pub fn a_star(
    start: Puzzle,
    goal: Puzzle,
    h: Box<dyn Fn(&Puzzle, &Puzzle) -> u32>,
) -> Option<Vec<Puzzle>> {
    let mut open_list = BinaryHeap::new();
    let mut closed_set = vec![];
    open_list.push(start);

    while let Some(current_node) = open_list.pop() {
        if current_node == goal {
            let path = reverse_path(current_node);
            return Some(path);
        }
        closed_set.push(current_node);
        let successors = current_node.neighbors();
        for successor in successors {
            if closed_set.contains(&successor) {
                continue;
            }
            let tentative_g = current_node.g + 1;
            let is_open_listed = open_list.contains(successor);
            if  is_open_listed && tentative_g >= successor.g {
                continue;
            }
            successor.previous = Some(Box::new(current_node));
            successor.g = tentative_g;
            successor.f = tentative_g + h(&successor, &goal);
            if !is_open_listed {
                open_list.push(successor);
            }
        }
    }
    None
}
