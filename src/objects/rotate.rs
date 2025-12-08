use crate::objects::{Aabb, Hit, Hittable};
use crate::prelude::*;

pub struct Rotated {
    object: Box<dyn Hittable>,
    axis: Axis,
    pivot: Point3,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl Rotated {
    pub fn new(object: impl Hittable + 'static, pivot: Point3, axis: Axis, degrees: f64) -> Self {
        let theta = degrees.to_radians();
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for x in [bbox.x.min, bbox.x.max] {
            for y in [bbox.y.min, bbox.y.max] {
                for z in [bbox.z.min, bbox.z.max] {
                    let corner = Point3::new(x, y, z);
                    let rotated = get_rotated(corner, pivot, axis, sin_theta, cos_theta);

                    for ax in Axis::AXES {
                        *min.axis_as_mut(ax) = min.axis(ax).min(rotated.axis(ax));
                        *max.axis_as_mut(ax) = max.axis(ax).max(rotated.axis(ax));
                    }
                }
            }
        }

        Self {
            object: Box::new(object),
            pivot,
            axis,
            sin_theta,
            cos_theta,
            bbox: Aabb::from_corners(min, max),
        }
    }
}

impl Hittable for Rotated {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let reference_point = ray.at(1.0);
        let rotated_origin = get_rotated(
            ray.origin,
            self.pivot,
            self.axis,
            -self.sin_theta, // rotates clockwise by theta
            self.cos_theta,
        );
        let rotated_reference = get_rotated(
            reference_point,
            self.pivot,
            self.axis,
            -self.sin_theta, // rotates clockwise by theta
            self.cos_theta,
        );
        let rotated_ray =
            Ray::with_time(rotated_origin, rotated_reference - rotated_origin, ray.time);

        self.object.hit(&rotated_ray, t_range).map(|mut hit| {
            let new_point = get_rotated(
                hit.point,
                self.pivot,
                self.axis,
                self.sin_theta,
                self.cos_theta,
            );
            let reference_point = hit.point + hit.normal;
            let new_reference_point = get_rotated(
                reference_point,
                self.pivot,
                self.axis,
                self.sin_theta,
                self.cos_theta,
            );
            let new_normal = new_reference_point - new_point;
            hit.point = new_point;
            hit.normal = new_normal;
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct Rotating {
    object: Box<dyn Hittable>,
    axis: Axis,
    pivot: Point3,
    angle: Lerp<f64>,
    bbox: Aabb,
}

impl Rotating {
    pub fn new(
        object: impl Hittable + 'static,
        pivot: Point3,
        axis: Axis,
        degrees1: f64,
        degrees2: f64,
    ) -> Self {
        let theta1 = degrees1.to_radians();
        let theta2 = degrees2.to_radians();
        let bbox = object.bounding_box();

        let mut global_min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut global_max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for x in [bbox.x.min, bbox.x.max] {
            for y in [bbox.y.min, bbox.y.max] {
                for z in [bbox.z.min, bbox.z.max] {
                    let corner = Point3::new(x, y, z);
                    let (min_corner, max_corner) =
                        swept_aabb_3d(corner, pivot, axis, theta1, theta2);

                    for ax in Axis::AXES {
                        *global_min.axis_as_mut(ax) = global_min.axis(ax).min(min_corner.axis(ax));
                        *global_max.axis_as_mut(ax) = global_max.axis(ax).max(max_corner.axis(ax));
                    }
                }
            }
        }

        Self {
            object: Box::new(object),
            pivot,
            axis,
            angle: Lerp::new(theta1, theta2),
            bbox: Aabb::from_corners(global_min, global_max),
        }
    }
}

impl Hittable for Rotating {
    fn hit(&self, ray: &Ray, t_range: Interval) -> Option<Hit> {
        let theta = self.angle.at(ray.time);
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let reference_point = ray.at(1.0);
        let rotated_origin = get_rotated(
            ray.origin, self.pivot, self.axis, -sin_theta, // rotates clockwise by theta
            cos_theta,
        );
        let rotated_reference = get_rotated(
            reference_point,
            self.pivot,
            self.axis,
            -sin_theta, // rotates clockwise by theta
            cos_theta,
        );
        let rotated_ray =
            Ray::with_time(rotated_origin, rotated_reference - rotated_origin, ray.time);

        self.object.hit(&rotated_ray, t_range).map(|mut hit| {
            let new_point = get_rotated(hit.point, self.pivot, self.axis, sin_theta, cos_theta);
            let reference_point = hit.point + hit.normal;
            let new_reference_point =
                get_rotated(reference_point, self.pivot, self.axis, sin_theta, cos_theta);
            let new_normal = new_reference_point - new_point;
            hit.point = new_point;
            hit.normal = new_normal;
            hit
        })
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

// rotated along axis counterclockwise
fn get_rotated(point: Point3, pivot: Point3, axis: Axis, sin_theta: f64, cos_theta: f64) -> Point3 {
    let mut rotated = point;
    let prev_axis = axis.prev();
    let next_axis = axis.next();
    let prev_axis_pivot = pivot.axis(prev_axis);
    let next_axis_pivot = pivot.axis(next_axis);
    let prev_axis_val = rotated.axis(prev_axis);
    let next_axis_val = rotated.axis(next_axis);
    // Rotate corner
    *rotated.axis_as_mut(prev_axis) = cos_theta * (prev_axis_val - prev_axis_pivot)
        + sin_theta * (next_axis_val - next_axis_pivot)
        + prev_axis_pivot;
    *rotated.axis_as_mut(next_axis) = -sin_theta * (prev_axis_val - prev_axis_pivot)
        + cos_theta * (next_axis_val - next_axis_pivot)
        + next_axis_pivot;
    rotated
}

// by ChatGPT
fn swept_aabb_3d(
    corner: Point3,
    pivot: Point3,
    axis: Axis,
    theta1: f64,
    theta2: f64,
) -> (Point3, Point3) {
    let prev_axis = axis.prev();
    let next_axis = axis.next();

    // Candidate angles where x (prev_axis) or y (next_axis) extrema occur
    let theta_prev = (-(corner.axis(next_axis) - pivot.axis(next_axis))
        / (corner.axis(prev_axis) - pivot.axis(prev_axis)))
    .atan(); // simplified, might use atan2
    let theta_next = ((corner.axis(prev_axis) - pivot.axis(prev_axis))
        / (corner.axis(next_axis) - pivot.axis(next_axis)))
    .atan();

    // Include endpoints
    let mut angles = vec![theta1, theta2];

    // Add extrema angles if they are inside [theta1, theta2]
    for theta in [theta_prev, theta_next] {
        let t = theta % (2.0 * std::f64::consts::PI);
        let t1 = theta1 % (2.0 * std::f64::consts::PI);
        let t2 = theta2 % (2.0 * std::f64::consts::PI);
        let t2 = if t2 < t1 {
            t2 + 2.0 * std::f64::consts::PI
        } else {
            t2
        };
        let t = if t < t1 {
            t + 2.0 * std::f64::consts::PI
        } else {
            t
        };
        if t1 <= t && t <= t2 {
            angles.push(theta);
        }
    }

    let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
    let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

    for theta in angles {
        let sin_theta = theta.sin();
        let cos_theta = theta.cos();
        let rotated = get_rotated(corner, pivot, axis, sin_theta, cos_theta);

        for ax in Axis::AXES {
            *min.axis_as_mut(ax) = min.axis(ax).min(rotated.axis(ax));
            *max.axis_as_mut(ax) = max.axis(ax).max(rotated.axis(ax));
        }
    }

    (min, max)
}
