use crate::float;
use crate::object::{Collision, Object};
use crate::ray::Ray;
use crate::scene::Scene;
use crate::types::Point3;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    min: Point3,
    max: Point3,
}

impl BoundingBox {
    pub fn axis_aligned(
        xmin: f32,
        xmax: f32,
        ymin: f32,
        ymax: f32,
        zmin: f32,
        zmax: f32,
    ) -> Self {
        BoundingBox {
            min: Point3::new(xmin, ymin, zmin),
            max: Point3::new(xmax, ymax, zmax),
        }
    }

    pub fn intersects(&self, ray: Ray) -> bool {
        let mut tmin = std::f32::NEG_INFINITY;
        let mut tmax = std::f32::INFINITY;
        for i in 0..3 {
            let origin = ray.origin[i];
            let dir = ray.dir[i];
            if dir != 0.0 {
                let t1 = (self.min[i] - origin) / dir;
                let t2 = (self.max[i] - origin) / dir;
                tmin = float::max(tmin, float::min(t1, t2));
                tmax = float::min(tmax, float::max(t1, t2));
            } else if origin < self.min[i] || origin > self.max[i] {
                return false;
            }
        }
        tmin <= tmax && tmax > 0.0
    }

    fn merge(left: &Self, right: &Self) -> Self {
        BoundingBox {
            min: Point3::new(
                float::min(left.min.x(), right.min.x()),
                float::min(left.min.y(), right.min.y()),
                float::min(left.min.z(), right.min.z()),
            ),
            max: Point3::new(
                float::max(left.max.x(), right.max.x()),
                float::max(left.max.y(), right.max.y()),
                float::max(left.max.z(), right.max.z()),
            ),
        }
    }

    fn volume(&self) -> f32 {
        let x = self.max.x() - self.min.x();
        let y = self.max.y() - self.min.y();
        let z = self.max.z() - self.min.z();
        x * y * z
    }
}

#[derive(Debug)]
pub struct BoundingVolumeHierarchy<'a> {
    root: Box<BvhNode<'a>>,
}

impl<'a> BoundingVolumeHierarchy<'a> {
    pub fn new(scene: &'a Scene) -> Self {
        assert!(!scene.objects.is_empty());

        let mut nodes = Vec::new();
        for object in &scene.objects {
            let node = Box::new(BvhNode::Leaf(object));
            let bounding_box = object.surface.bounding_box();
            nodes.push((node, bounding_box));
        }

        while nodes.len() > 1 {
            let mut new_nodes = Vec::new();
            while nodes.len() > 1 {
                let left = nodes.pop().unwrap();
                let mut min_i = 0;
                let mut min_volume = std::f32::INFINITY;
                for (i, node) in nodes.iter().enumerate().skip(1) {
                    let volume = BoundingBox::merge(&left.1, &node.1).volume();
                    if volume < min_volume {
                        min_i = i;
                        min_volume = volume;
                    }
                }
                let right = nodes.swap_remove(min_i);
                let new_box = BoundingBox::merge(&left.1, &right.1);
                let new_node =
                    Box::new(BvhNode::Node(new_box.clone(), left.0, right.0));
                new_nodes.push((new_node, new_box));
            }
            nodes.extend(new_nodes);
        }

        assert_eq!(nodes.len(), 1);
        BoundingVolumeHierarchy {
            root: nodes.remove(0).0,
        }
    }

    pub fn collide(&self, ray: Ray) -> Option<Collision> {
        self.root.collide(ray)
    }
}

#[derive(Debug)]
enum BvhNode<'a> {
    Node(BoundingBox, Box<BvhNode<'a>>, Box<BvhNode<'a>>),
    Leaf(&'a Object),
}

impl<'a> BvhNode<'a> {
    fn collide(&self, ray: Ray) -> Option<Collision> {
        match *self {
            BvhNode::Node(ref bb, ref left, ref right) => {
                if bb.intersects(ray) {
                    let left = left.collide(ray);
                    let right = right.collide(ray);
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
            BvhNode::Leaf(obj) => obj.collide(ray),
        }
    }
}
