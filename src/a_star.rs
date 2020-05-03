use super::puzzle::Puzzle;

pub fn a_star(
    start: Puzzle,
    end: Puzzle,
    h: Box<dyn Fn(&Puzzle, &Puzzle) -> u32>,
) -> Option<Vec<Puzzle>> {
    let mut open_set: Vec<Box<Puzzle>> = vec![];
    let mut closed_set: Vec<Box<Puzzle>> = vec![];
    open_set.push(Box::new(start));

    while !open_set.is_empty() {
        let mut best = 0;
        for i in 0..open_set.len() {
            if open_set[i].f < open_set[best].f {
                best = i;
            }
        }
        let current = open_set.remove(best);
        if *current == end {
            let mut path: Vec<Puzzle> = vec![];
            let mut cur = *current;
            path.push(cur.clone());
            while let Some(prev) = *(cur.previous) {
                path.push(prev.clone());
                cur = prev;
            }
            return Some(path);
        }
        let neighbors = current.neighbors();
        for mut neighbor in neighbors {
            if !closed_set.contains(&neighbor) {
                let g = current.g + 1;
                if open_set.contains(&neighbor) {
                    if g < neighbor.g {
                        neighbor.g = g;
                        neighbor.h = h(&neighbor, &end);
                        neighbor.f = neighbor.g + neighbor.h;
                        neighbor.previous =
                            Box::new(Some((*current).clone()));
                    }
                } else {
                    neighbor.g = g;
                    neighbor.h = h(&neighbor, &end);
                    neighbor.f = neighbor.g + neighbor.h;
                    neighbor.previous = Box::new(Some((*current).clone()));
                    open_set.push(neighbor.clone());
                }
            }
        }

        closed_set.push(current);
    }
    None
}
