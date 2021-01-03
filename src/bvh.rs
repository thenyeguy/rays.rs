use crate::bounds::BoundingBox;
use crate::object::{Object, Sample};
use crate::ray::Ray;
use crate::scene::Scene;

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
                    let volume = BoundingBox::union(&left.1, &node.1).volume();
                    if volume < min_volume {
                        min_i = i;
                        min_volume = volume;
                    }
                }
                let right = nodes.swap_remove(min_i);
                let new_box = BoundingBox::union(&left.1, &right.1);
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

    pub fn sample(&self, ray: Ray) -> Option<Sample> {
        self.root.sample(ray)
    }
}

#[derive(Debug)]
enum BvhNode<'a> {
    Node(BoundingBox, Box<BvhNode<'a>>, Box<BvhNode<'a>>),
    Leaf(&'a Object),
}

impl<'a> BvhNode<'a> {
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
            BvhNode::Leaf(obj) => obj.sample(ray),
        }
    }
}
