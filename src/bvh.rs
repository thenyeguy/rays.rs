use crate::bounds::BoundingBox;
use crate::float;
use crate::object::{Object, Sample};
use crate::ray::Ray;
use crate::types::Axis;

#[derive(Debug)]
pub struct BoundingVolumeHierarchy {
    root: Box<BvhNode>,
}

impl BoundingVolumeHierarchy {
    pub fn new(objects: Vec<Object>) -> Self {
        let objects = objects
            .into_iter()
            .map(|obj| (obj.surface.bounding_box(), obj))
            .collect();
        BoundingVolumeHierarchy {
            root: BvhNode::new(objects),
        }
    }

    pub fn sample(&self, ray: Ray) -> Option<Sample> {
        self.root.sample(ray)
    }
}

#[derive(Debug)]
enum BvhNode {
    Node(BoundingBox, Box<BvhNode>, Box<BvhNode>),
    Leaf(Object),
}

impl BvhNode {
    fn new(mut objects: Vec<(BoundingBox, Object)>) -> Box<Self> {
        if objects.len() == 1 {
            Box::new(BvhNode::Leaf(objects.remove(0).1))
        } else {
            let bb = objects
                .iter()
                .map(|(bb, _obj)| bb)
                .fold(BoundingBox::empty(), |ref left, ref right| {
                    BoundingBox::union(left, right)
                });
            let (left, right) = partition(objects);
            Box::new(BvhNode::Node(bb, BvhNode::new(left), BvhNode::new(right)))
        }
    }

    fn sample(&self, ray: Ray) -> Option<Sample> {
        match *self {
            BvhNode::Node(ref bb, ref left, ref right) => {
                if bb.intersects(ray) {
                    let left = left.sample(ray);
                    let right = right.sample(ray);
                    match (left, right) {
                        (Some(lc), Some(rc)) => {
                            if lc.intersection.distance
                                < rc.intersection.distance
                            {
                                Some(lc)
                            } else {
                                Some(rc)
                            }
                        }
                        (Some(c), _) => Some(c),
                        (_, Some(c)) => Some(c),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            BvhNode::Leaf(ref obj) => obj.sample(ray),
        }
    }
}

const NUM_BINS: usize = 8;

#[derive(Debug)]
struct Bin {
    bb: BoundingBox,
    count: usize,
}

impl Bin {
    fn empty() -> Self {
        Bin {
            bb: BoundingBox::empty(),
            count: 0,
        }
    }

    fn combined(bins: &[Self]) -> Self {
        let mut result = Bin::empty();
        for bin in bins {
            result.bb.merge(&bin.bb);
            result.count += bin.count;
        }
        result
    }
}

fn partition(
    objects: Vec<(BoundingBox, Object)>,
) -> (Vec<(BoundingBox, Object)>, Vec<(BoundingBox, Object)>) {
    let centroid_bb = objects.iter().map(|(bb, _obj)| bb.centroid()).fold(
        BoundingBox::empty(),
        |mut bb, centroid| {
            bb.add_point(centroid);
            bb
        },
    );
    let axis = widest_axis(&centroid_bb);
    let (axis_min, axis_max) = (centroid_bb.min[axis], centroid_bb.max[axis]);
    let bin_size = (axis_max - axis_min) / NUM_BINS as f32;

    if bin_size < float::EPSILON {
        let mut left = objects;
        let right = left.split_off(left.len() / 2);
        (left, right)
    } else {
        let mut assignments = Vec::with_capacity(objects.len());
        let mut bins: Vec<_> = (0..NUM_BINS).map(|_| Bin::empty()).collect();
        for (bb, _) in objects.iter() {
            let centroid = bb.centroid();
            for (i, bin) in bins.iter_mut().enumerate() {
                let boundary = axis_min + bin_size * (i + 1) as f32;
                if centroid[axis] <= boundary + float::EPSILON {
                    assignments.push(i);
                    bin.bb.merge(&bb);
                    bin.count += 1;
                    break;
                }
            }
        }

        let partition_idx = best_partition(&bins);
        let mut left = Vec::new();
        let mut right = Vec::new();
        for (i, obj) in objects.into_iter().enumerate() {
            if assignments[i] < partition_idx {
                left.push(obj);
            } else {
                right.push(obj);
            }
        }
        (left, right)
    }
}

fn widest_axis(bb: &BoundingBox) -> Axis {
    let extent = bb.max - bb.min;
    if extent[0] > extent[1] && extent[0] > extent[2] {
        Axis::X
    } else if extent[1] > extent[2] {
        Axis::Y
    } else {
        Axis::Z
    }
}

fn best_partition(bins: &[Bin]) -> usize {
    let mut min_partition = 0;
    let mut min_sah = std::f32::INFINITY;
    for partition in 1..(NUM_BINS - 1) {
        let left = Bin::combined(&bins[..partition]);
        let right = Bin::combined(&bins[partition..]);
        let sah = left.bb.surface_area() * left.count as f32
            + right.bb.surface_area() * right.count as f32;
        if sah < min_sah {
            min_partition = partition;
            min_sah = sah;
        }
    }
    min_partition
}
