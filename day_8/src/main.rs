use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
struct JunctionId(usize);

#[derive(Debug, Clone)]
struct Junction {
    x: u64,
    y: u64,
    z: u64,
    id: JunctionId,
}

impl Junction {
    fn distance_sq(&self, other: &Self) -> u64 {
        // lambda to handle underflow
        let underflow_diff = |x1: u64, x2: u64| -> u64 {
            let abs_diff = x1.max(x2) - x1.min(x2);
            abs_diff * abs_diff
        };

        let mut result = underflow_diff(self.x, other.x);
        result += underflow_diff(self.y, other.y);
        result += underflow_diff(self.z, other.z);
        result
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Connection {
    distance_sq: u64,
    junctions: (JunctionId, JunctionId),
}

impl Ord for Connection {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance_sq.cmp(&self.distance_sq)
    }
}

impl PartialOrd for Connection {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Default, Clone)]
struct Circuit {
    junctions: HashSet<JunctionId>,
}

impl PartialEq for Circuit {
    fn eq(&self, other: &Self) -> bool {
        self.size() == other.size()
    }
}

impl Eq for Circuit {}

impl Ord for Circuit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.size().cmp(&other.size())
    }
}

impl PartialOrd for Circuit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Circuit {
    fn size(&self) -> usize {
        self.junctions.len()
    }

    fn contains(&self, id: &JunctionId) -> bool {
        self.junctions.contains(id)
    }

    fn merge(&mut self, other: Self) {
        self.junctions.extend(other.junctions.into_iter())
    }

    fn add_conn(&mut self, conn: Connection) {
        self.junctions.insert(conn.junctions.0);
        self.junctions.insert(conn.junctions.1);
    }
}

#[derive(Debug, Default, Clone)]
struct Circuits(BinaryHeap<Circuit>);

impl Circuits {
    // adds a connection if two junctions are not already connected
    // and returns true.
    //
    // If the junctions are already connected it returns false
    fn add_connection(&mut self, conn: Connection) -> bool {
        if self.already_connected(&conn) {
            return false;
        }

        let mut new_circuits = Circuits::default();
        let mut circuit_left = None;
        let mut circuit_right = None;
        while let Some(circuit) = self.0.pop() {
            if circuit.contains(&conn.junctions.0) {
                assert!(circuit_left.is_none());
                circuit_left = Some(circuit);
                continue;
            }

            if circuit.contains(&conn.junctions.1) {
                assert!(circuit_right.is_none());
                circuit_right = Some(circuit);
                continue;
            }
            new_circuits.0.push(circuit);
        }

        match (circuit_left, circuit_right) {
            (Some(mut l), Some(r)) => {
                l.merge(r);
                new_circuits.0.push(l);
            }
            (Some(mut c), None) => {
                c.add_conn(conn);
                new_circuits.0.push(c);
            }
            (None, Some(mut c)) => {
                c.add_conn(conn);
                new_circuits.0.push(c)
            }
            (None, None) => {
                // neither junction exist in a circuit, we have to create one
                let mut new_circuit = Circuit::default();
                new_circuit.add_conn(conn);
                new_circuits.0.push(new_circuit);
            }
        }
        self.0 = new_circuits.0;
        true
    }

    fn already_connected(&self, conn: &Connection) -> bool {
        self.0.iter().any(|circuit| {
            circuit.contains(&conn.junctions.0) && circuit.contains(&conn.junctions.1)
        })
    }
}

#[derive(Debug, Default, Clone)]
struct Factory {
    #[allow(dead_code)]
    junctions: Vec<Junction>,
    connections: BinaryHeap<Connection>,
    circuits: Circuits,
}

impl Factory {
    fn from_file(fname: impl AsRef<Path>) -> Self {
        let f = File::open(fname.as_ref()).unwrap();
        let reader = BufReader::new(f);

        let mut junctions: Vec<Junction> = vec![];

        for (id, line) in reader.lines().enumerate() {
            let id = JunctionId(id);
            let line = line.unwrap();
            let mut iter = line.split(',');
            let x = iter.next().unwrap().parse().unwrap();
            let y = iter.next().unwrap().parse().unwrap();
            let z = iter.next().unwrap().parse().unwrap();
            junctions.push(Junction { x, y, z, id })
        }

        let mut connections = BinaryHeap::new();
        for (i, b1) in junctions.iter().enumerate() {
            for b2 in &junctions[i + 1..] {
                let distance_sq = b1.distance_sq(b2);
                connections.push(Connection {
                    distance_sq,
                    junctions: (b1.id, b2.id),
                })
            }
        }
        Self {
            junctions,
            connections,
            circuits: Circuits::default(),
        }
    }

    fn part_a(mut self, num_to_join: usize) -> usize {
        let mut added_connections = 0;
        while added_connections < num_to_join {
            let conn = self.connections.pop().unwrap();
            self.circuits.add_connection(conn);
            added_connections += 1;
        }
        let mut prod = 1;
        for _ in 0..3 {
            prod *= self.circuits.0.pop().unwrap().size();
        }
        prod
    }

    fn part_b(mut self) -> u64 {
        let mut conn = None;
        while self.circuits.0.peek().map(|c| c.size()).unwrap_or_default() < self.junctions.len() {
            conn = Some(self.connections.pop().unwrap());
            self.circuits.add_connection(conn.clone().unwrap());
        }

        let (l, r) = conn.unwrap().junctions;
        self.junctions[l.0].x * self.junctions[r.0].x
    }
}

fn main() {
    let factory = Factory::from_file("input.txt");

    println!("Part A: `{}`", factory.clone().part_a(1000));
    println!("Part B: `{}`", factory.part_b());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let ws = Factory::from_file("test.txt");
        assert_eq!(40, ws.part_a(10))
    }

    #[test]
    fn test_b() {
        let ws = Factory::from_file("test.txt");
        assert_eq!(25272, ws.part_b())
    }
}
