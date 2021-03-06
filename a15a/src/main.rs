use std::{
    cell::RefCell,
    collections::HashMap,
    io::{self, BufRead},
};

fn main() {
    let stdin = io::stdin();
    let lines = stdin.lock().lines();
    let mut grid: HashMap<(i32, i32), RefCell<Node>> = HashMap::new();
    let mut h = 0;
    let mut w = 0;
    for line in lines {
        w = 0;
        let l = line.unwrap();
        let row = l.chars().map(|c| c.to_digit(10).unwrap());
        for c in row {
            let n = RefCell::new(Node {
                cost: c as i32,
                dist: None,
                heur: 1,
                closed: false,
            });
            grid.insert((w, h), n);
            w += 1;
        }
        h += 1;
    }
    for (p, c) in grid.iter() {
        c.borrow_mut().heur = w - p.0 + h - p.1;
    }

    let start = grid.get(&(0, 0)).unwrap();
    {
        let mut start = start.borrow_mut();
        start.dist = Some(0);
    }
    let mut current: HashMap<(i32, i32), &RefCell<Node>> = HashMap::new();
    current.insert((0, 0), start);
    loop {
        let (p, best) = current
            .iter()
            .filter(|&(_, c)| !c.borrow().closed && c.borrow().dist != None)
            .min_by_key(|&(_, c)| c.borrow().dist.unwrap() + c.borrow().heur)
            .unwrap();
        best.borrow_mut().closed = true;
        let mut next: HashMap<(i32, i32), &RefCell<Node>> = HashMap::new();
        for adj_best in [
            (p.0, p.1 - 1),
            (p.0, p.1 + 1),
            (p.0 + 1, p.1),
            (p.0 - 1, p.1),
        ]
        .iter()
        {
            if let Some(exists) = grid.get(adj_best) {
                next.insert(*adj_best, exists);
            }
        }
        for (p, c) in next.iter() {
            {
                let mut e = c.borrow_mut();
                for np in [
                    (p.0, p.1 - 1),
                    (p.0, p.1 + 1),
                    (p.0 + 1, p.1),
                    (p.0 - 1, p.1),
                ]
                .iter()
                {
                    if let Some(back) = grid.get(np) {
                        let b = back.borrow();
                        if let Some(dist) = b.dist {
                            e.dist = Some(match e.dist {
                                Some(d) => i32::min(d, dist + e.cost),
                                None => dist + e.cost,
                            })
                        }
                    }
                }
                if p.0 == w - 1 && p.1 == h - 1 {
                    println!("{}", e.dist.unwrap());
                    return;
                }
            }
        }
        current.extend(next.into_iter());
    }
}

struct Node {
    dist: Option<i32>,
    cost: i32,
    heur: i32,
    closed: bool,
}
