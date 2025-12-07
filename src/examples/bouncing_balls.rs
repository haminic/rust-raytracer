use crate::camera::*;
use crate::materials::*;
use crate::objects::*;
use crate::prelude::*;

pub fn bouncing_balls(n: i32) -> (World, Camera) {
    let mut world = HittableList::new();

    let ground_material: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -n..n {
        for b in -n..n {
            let choose_mat = random_unit_f64();
            let choose_bounce = random_unit_f64();
            let center = Point3::new(
                a as f64 + 0.9 * random_unit_f64(),
                0.2,
                b as f64 + 0.9 * random_unit_f64(),
            );
            let translation = Vec3::new(0.0, random_range(0.0..0.5), 0.0);

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = match choose_mat {
                    x if x < 0.5 => {
                        let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                        Arc::new(Lambertian::new(albedo))
                    }
                    x if x < 0.8 => {
                        let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                        Arc::new(Metal::new(albedo))
                    }
                    _ => Arc::new(Dielectric::new(1.5)),
                };
                let sphere = Sphere::new(center, 0.2, sphere_material);
                if choose_bounce > 0.5 {
                    world.add(sphere);
                } else {
                    world.add(Translated::new_moving(
                        sphere,
                        Vec3::new(0.0, 0.0, 0.0),
                        translation,
                    ));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Sphere::new(Point3::new(2.0, 1.0, -2.5), 1.0, material2));

    let material3 = Arc::new(Metal::new(Metal::GOLD_ALBEDO));
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material3));

    let world = Bvh::from_list(world);

    let resolution = Resolution::with_aspect_ratio(16.0 / 9.0, 1200);
    let cam = Camera::new(
        resolution,
        Point3::new(10.0, 2.0, 4.0),
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        10.0,
        0.6,
    );

    (
        World::new(Color::new(0.70, 0.80, 1.00), Box::new(world)),
        cam,
    )
}
