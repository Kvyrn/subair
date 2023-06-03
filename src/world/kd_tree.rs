use bevy::prelude::*;
use tracing::instrument;

#[instrument(skip(tree, point_consumer))]
pub fn points_in_range(
    tree: &Tree3d,
    searched_point: Vec3,
    range: f32,
    mut point_consumer: impl FnMut((Vec3, usize)),
) {
    search_for_close_points(tree, searched_point, range, &mut point_consumer);
}

fn search_for_close_points(
    node: &Tree3d,
    searched_point: Vec3,
    range: f32,
    point_consumer: &mut impl FnMut((Vec3, usize)),
) {
    match node {
        Tree3d::Leaf(point) => {
            if point.0.distance_squared(searched_point) < range {
                point_consumer(*point)
            }
        }
        Tree3d::Branch {
            point,
            children,
            axis,
        } => {
            if point.0.distance_squared(searched_point) < range * range {
                point_consumer(*point)
            }

            let distance = signed_distance_on_axis(point.0, searched_point, *axis);
            // If less than range away from boundry both branches must be searched
            if distance.abs() < range {
                for child in children.iter() {
                    search_for_close_points(child, searched_point, range, point_consumer);
                }
            } else if distance < 0.0 {
                search_for_close_points(&children[0], searched_point, range, point_consumer);
            } else {
                search_for_close_points(&children[1], searched_point, range, point_consumer);
            }
        }
        Tree3d::SingleChild {
            point,
            child,
            axis: _,
        } => {
            if point.0.distance_squared(searched_point) < range * range {
                point_consumer(*point)
            }
            // There should only ever be one leaf left in this branch so this
            // operation is not expensive
            search_for_close_points(child, searched_point, range, point_consumer);
        }
    }
}

fn signed_distance_on_axis(point1: Vec3, point2: Vec3, axis: Axis) -> f32 {
    match axis {
        Axis::X => point1.x - point2.x,
        Axis::Y => point1.y - point2.y,
        Axis::Z => point1.z - point2.z,
    }
}

pub fn construct_tree(points: &[Vec3]) -> Tree3d {
    // Point indices must be remembered
    let mut points: Vec<_> = points
        .iter()
        .copied()
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect();
    tree_branch(&mut points, Axis::X)
}

fn tree_branch(points: &mut [(Vec3, usize)], axis: Axis) -> Tree3d {
    if points.len() == 1 {
        return Tree3d::Leaf(points[0]);
    }
    sort_points_on_axis(points, axis);
    if points.len() == 2 {
        return Tree3d::SingleChild {
            point: points[0],
            child: Box::new(Tree3d::Leaf(points[1])),
            axis,
        };
    }
    let midpoint = points.len() / 2;
    let point = points[midpoint];
    let (lesser_points, points_ge) = points.split_at_mut(midpoint);
    let greater_points = &mut points_ge[1..];

    Tree3d::Branch {
        point,
        children: Box::new([
            tree_branch(lesser_points, axis.next()),
            tree_branch(greater_points, axis.next()),
        ]),
        axis,
    }
}

#[derive(Debug, Clone)]
pub enum Tree3d {
    Leaf((Vec3, usize)),
    Branch {
        point: (Vec3, usize),
        // First lesser than greater
        children: Box<[Tree3d; 2]>,
        axis: Axis,
    },
    SingleChild {
        point: (Vec3, usize),
        // Greater point
        child: Box<Tree3d>,
        axis: Axis,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Axis {
    fn next(&self) -> Axis {
        match self {
            Axis::X => Axis::Y,
            Axis::Y => Axis::Z,
            Axis::Z => Axis::X,
        }
    }
}

fn sort_points_on_axis(points: &mut [(Vec3, usize)], axis: Axis) {
    // Comparison fails if point is NaN
    match axis {
        Axis::X => points.sort_by(|a, b| a.0.x.partial_cmp(&b.0.x).unwrap()),
        Axis::Y => points.sort_by(|a, b| a.0.y.partial_cmp(&b.0.y).unwrap()),
        Axis::Z => points.sort_by(|a, b| a.0.z.partial_cmp(&b.0.z).unwrap()),
    }
}
