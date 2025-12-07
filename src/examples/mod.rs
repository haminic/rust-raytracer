mod bouncing_balls;
mod cornell_box;
mod simple_light;
mod test_fog;
mod cornell_smoke;

#[allow(dead_code)]
pub use bouncing_balls::bouncing_balls;
#[allow(dead_code)]
pub use cornell_box::cornell_box;
#[allow(dead_code)]
pub use simple_light::simple_light;
#[allow(dead_code)]
pub use test_fog::test_fog;
#[allow(dead_code)]
pub use cornell_smoke::cornell_smoke;

use crate::prelude::*;
use crate::render::*;
use crate::materials::*;
use crate::objects::*;