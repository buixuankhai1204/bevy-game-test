use bevy::prelude::*;

pub fn calculate_bounce(velocity: Vec2, normal: Vec2) -> Vec2 {
    // Reflect velocity using normal vector
    let dot_product = velocity.dot(normal);
    velocity - 2.0 * dot_product * normal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_bounce() {
        let velocity = Vec2::new(1.0, -1.0);
        let normal = Vec2::new(0.0, 1.0);
        let result = calculate_bounce(velocity, normal);
        assert_eq!(result, Vec2::new(1.0, 1.0));
    }
}