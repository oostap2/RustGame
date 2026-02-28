pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

pub fn check_circle_point_collision(
    circle_center: &Vector2,
    circle_radius: &f32,
    point: &Vector2,
) -> bool {
    let distance_squared =
        (circle_center.x - point.x).powi(2) + (circle_center.y - point.y).powi(2);
    distance_squared <= circle_radius.powi(2)
}
