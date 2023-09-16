use alloc::vec::Vec;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub point_vec: Vec<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct L2Dist {
    pub points: Vec<Point>,
    pub dist: f64,
}

pub fn compute_l2_distance(p1: &Point, p2: &Point) -> L2Dist {
    let mut sum = 0.0;
    for i in 0..p1.point_vec.len() {
        sum += (p1.point_vec[i] - p2.point_vec[i]) * (p1.point_vec[i] - p2.point_vec[i]);
    }
    let result = L2Dist {
        points: vec![p1.clone(), p2.clone()],
        dist: libm::sqrt(sum),
    };
    result
}

#[cfg(test)]
mod test {
    use super::{Point, compute_l2_distance};

    #[test]
    fn test_l2_dist() {
        let a = Point{point_vec: vec![1.0, 0.0]};
        let b = Point{point_vec: vec![0.0, 0.0]};

        let res = compute_l2_distance(&a, &b);

        println!("{:?}", res)
    }
}