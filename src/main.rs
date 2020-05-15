use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::collections::BTreeMap;

#[derive(Debug, Clone)]
struct Range{
    start: i32,
    end: i32,
}

impl std::cmp::Eq for Range{
    
}
impl std::cmp::PartialEq for Range{
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl std::cmp::PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Range {
    fn cmp(&self, other: &Self) -> Ordering {
        println!("run cmp: {:?} {:?}", self, other);
        return if self.end < other.end {
            Ordering::Greater
        } else if other.end < self.end {
            Ordering::Less
        } else{
            let range1 = self.end - self.start;
                let range2 = other.end - other.start;
                if range1 < range2 {
                    Ordering::Greater
                } else if range2 < range1 {
                    Ordering::Less
                } else {
                    //相等,比较谁的起始时间
                    if self.start < other.start {
                        Ordering::Greater
                    } else if other.start < self.start {
                        Ordering::Less
                    } else {
                        Ordering::Equal
                    }
                }
        };
    }

}

struct Solution{}
impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut heap = BinaryHeap::new();
        for e in events {
            println!("heap push");
            heap.push(Range {
                start: e[0],
                end: e[1],
            });
        }
        println!("heap: {:?}", heap);
        let mut m = BTreeMap::new();
        let mut count = 0;
        while let Some(r) = heap.pop() {
            println!("r={:?}", r);
            for i in r.start..r.end + 1 {
                use std::collections::btree_map::Entry;
                let e = m.entry(i);
                match e {
                    Entry::Occupied(_) => {
                        continue;
                    }
                    Entry::Vacant(o) => {
                        o.insert(1);
                        count += 1;
                        break;
                    }
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test() {
        let t = Solution::max_events(vec![vec![1, 3], vec![1, 3], vec![1, 3], vec![3, 4]]);
        assert_eq!(t, 4);
        // return;
        let t = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4]]);
        assert_eq!(t, 3);
        // return;
        let t = Solution::max_events(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 2]]);
        assert_eq!(t, 4);
        let t = Solution::max_events(vec![
            vec![1, 4],
            vec![4, 4],
            vec![2, 2],
            vec![3, 4],
            vec![1, 1],
        ]);
        assert_eq!(t, 4);
        let t = Solution::max_events(vec![vec![1, 100000]]);
        assert_eq!(t, 1);
        let t = Solution::max_events(vec![
            vec![1, 1],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4],
            vec![1, 5],
            vec![1, 6],
            vec![1, 7],
        ]);
        assert_eq!(t, 7);
    }
}

fn main() {
    println!("Hello, world!");
    let t = Solution::max_events(vec![vec![1, 2], vec![2, 2], vec![3, 4], vec![1, 2]]);
    println!("max: {}", t);
}

