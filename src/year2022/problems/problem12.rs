use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

/// Grid Id
type VId = i32;

/// Grid Value
type Score = u8;

type Step = i32;

#[derive(Debug)]
pub struct Graph {
    pub rows: i32,
    pub cols: i32,
    pub vertices: HashMap<VId, Score>,
    pub adjacency: HashMap<VId, Vec<(VId, Score)>>,
}

impl Graph {
    const DR: [i32; 4] = [-1, 1, 0, 0];
    const DC: [i32; 4] = [0, 0, -1, 1];

    pub fn new(rows: i32, cols: i32) -> Self {
        Graph {
            rows,
            cols,
            vertices: HashMap::new(),
            adjacency: HashMap::new(),
        }
    }
    fn get_id(&self, vid_row: VId, vid_col: VId) -> VId {
        vid_row * self.cols + vid_col
    }
    /// VId: row * total_column + column
    pub fn add_node(&mut self, vid_row: VId, vid_col: VId, score: Score) {
        let vid_result = self.get_id(vid_row, vid_col);
        self.vertices.insert(vid_result, score);
    }
    pub fn add_edge(&mut self, vid_row: VId, vid_col: VId) {
        let vid_result = self.get_id(vid_row, vid_col);

        let mut vid_value = self.vertices.get(&vid_result).unwrap();

        if *vid_value == ('S' as u8) {
            vid_value = &('a' as u8);
        }

        if *vid_value == ('E' as u8) {
            vid_value = &('z' as u8);
        }

        for i in 0..4 {
            let nr = vid_row + Self::DR[i];
            let nc = vid_col + Self::DC[i];
            if nr < 0 || nc < 0 {
                continue;
            }
            if nr >= self.rows || nc >= self.cols {
                continue;
            }

            let neightbor_id = self.get_id(nr, nc);

            let adjacent_list = self.adjacency.entry(vid_result).or_default();

            if let Some(neightbor_value) = self.vertices.get(&neightbor_id) {
                let mut check_value = neightbor_value;
                if *check_value == ('S' as u8) {
                    check_value = &('a' as u8);
                }

                if *check_value == ('E' as u8) {
                    check_value = &('z' as u8);
                }

                let gap = *check_value as i32 - *vid_value as i32;

                // BUG: "if gap >= 0 && gap <= 1" the >=0 will cause error
                // NOTE:
                if gap <= 1 {
                    adjacent_list.push((neightbor_id, *neightbor_value))
                }
            }
        }
    }

    pub fn best_signal(&self, start_score: VId, end_score: VId) -> Option<Step> {
        let mut visited: HashMap<VId, bool> = HashMap::new();
        let mut path_cost: HashMap<VId, (VId, Step)> = HashMap::new();
        let mut queue: Vec<(VId, VId, Step)> = Vec::new();

        visited.insert(start_score, true);

        if let Some(adjacency_list) = self.adjacency.get(&start_score) {
            for neightbor in adjacency_list {
                let node = visited.get(&neightbor.0);
                if node.is_some_and(|&x| x != true) || node.is_none() {
                    visited.insert(neightbor.0, true);
                    queue.push((start_score, neightbor.0, 1));
                }
            }
        }

        while queue.len() != 0 && path_cost.get(&end_score).is_none() {
            let first_node = queue.remove(0);

            let check_item = first_node.1;

            // NOTE:  A, C, 1 => C : [A, 1]
            path_cost
                .entry(check_item)
                .or_insert((first_node.0, first_node.2));

            if let Some(check_adjacency) = self.adjacency.get(&check_item) {
                for neightbor in check_adjacency {
                    let node = visited.get(&neightbor.0);
                    if node.is_some_and(|&x| x != true) || node.is_none() {
                        visited.insert(neightbor.0, true);
                        queue.push((
                            check_item,
                            neightbor.0,
                            path_cost.get(&check_item).unwrap().1 + 1,
                        ));
                    }
                }
            }
        }

        // let mut path = Vec::new();
        // let mut check_key = &end_score;
        //
        // if path_cost.get(check_key).is_some() {
        //     path.push(check_key.clone());
        // }
        //
        // while path_cost.get(check_key).is_some() {
        //     if let Some(node) = path_cost.get(check_key) {
        //         check_key = &node.0;
        //         path.push(check_key.clone());
        //     }
        // }
        // path.reverse();
        //
        // for p in &path {
        //     let vb = *self.vertices.get(&p).unwrap() as char;
        // }

        Some(path_cost.get(&end_score)?.1)
    }
}

pub fn problem12_1(path: &str) -> i32 {
    let file = File::open(path).expect("no file exists");
    let reader = BufReader::new(file);

    // a: 97, z: 122
    let mut start_score = (0, 0);
    let mut end_score = (0, 0);

    let matrix = reader
        .lines()
        .map(|x| x.unwrap().as_bytes().to_vec())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut graph = Graph::new(rows as i32, cols as i32);

    for j in 0..rows {
        for i in 0..cols {
            if matrix[j][i] == ('S' as u8) {
                start_score = (j as i32, i as i32);
            }

            if matrix[j][i] == ('E' as u8) {
                end_score = (j as i32, i as i32);
            }

            graph.add_node(j as i32, i as i32, matrix[j][i])
        }
    }

    for j in 0..rows {
        for i in 0..cols {
            graph.add_edge(j as i32, i as i32)
        }
    }

    let start = graph.get_id(start_score.0, start_score.1);
    let end = graph.get_id(end_score.0, end_score.1);

    match graph.best_signal(start, end) {
        Some(step) => step,
        None => 0,
    }
}

pub fn problem12_2(path: &str) -> i32 {
    let file = File::open(path).expect("no file exists");
    let reader = BufReader::new(file);

    // a: 97, z: 122
    let mut end_score = (0, 0);

    let matrix = reader
        .lines()
        .map(|x| x.unwrap().as_bytes().to_vec())
        .collect::<Vec<_>>();

    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut graph = Graph::new(rows as i32, cols as i32);

    for j in 0..rows {
        for i in 0..cols {
            if matrix[j][i] == ('E' as u8) {
                end_score = (j as i32, i as i32);
            }

            graph.add_node(j as i32, i as i32, matrix[j][i])
        }
    }

    for j in 0..rows {
        for i in 0..cols {
            graph.add_edge(j as i32, i as i32)
        }
    }

    let mut start_point_list: Vec<(i32, i32)> = Vec::new();
    // TODO: get 'a' position:
    for j in 0..rows {
        for i in 0..cols {
            let check_pos = matrix[j][i];
            if check_pos == ('a' as u8) {
                start_point_list.push((j as i32, i as i32))
            }
        }
    }
    let mut step = i32::MAX;

    let end = graph.get_id(end_score.0, end_score.1);

    for start_coor in start_point_list {
        let start = graph.get_id(start_coor.0, start_coor.1);

        let check_result = graph.best_signal(start, end);
        if let Some(result) = check_result {
            if result < step {
                step = result;
            }
        }
    }

    step
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tests_y2022_d12_1() {
        let file_path_simple = "testdata/y2022_p12_simple.txt";
        assert_eq!(31, problem12_1(file_path_simple));

        let file_path = "testdata/y2022_p12.txt";
        assert_eq!(352, problem12_1(file_path));
    }

    #[test]
    fn tests_y2022_d12_2() {
        let file_path_simple = "testdata/y2022_p12_simple.txt";
        assert_eq!(29, problem12_2(file_path_simple));

        let file_path = "testdata/y2022_p12.txt";
        assert_eq!(345, problem12_2(file_path));
    }
}
