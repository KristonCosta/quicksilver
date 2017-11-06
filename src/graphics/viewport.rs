use geom::{Rectangle, Transform, Vector};

pub struct ViewportBuilder {
    screen_size: Vector
}

impl ViewportBuilder {
    pub fn new(screen_size: Vector) -> ViewportBuilder {
        ViewportBuilder { screen_size }
    }

    pub fn get_viewport(&self, world: Rectangle) -> Viewport {
        self.get_viewport_transformed(world, Transform::identity())
    }

    pub fn get_viewport_transformed(&self, world: Rectangle, transform: Transform) -> Viewport {
        let unproject = Transform::scale(self.screen_size.times(world.size().recip())) *
            Transform::translate(-world.top_left()) * transform;
        let project = unproject.inverse();
        Viewport { project, unproject }
    }

}

pub struct Viewport {
    project: Transform,
    unproject: Transform
}

impl Viewport {
    pub fn project(&self) -> Transform {
        self.project
    }

    pub fn unproject(&self) -> Transform {
        self.unproject
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn projection() {
        let builder = ViewportBuilder::new(Vector::newi(100, 100));
        let viewport = builder.get_viewport(Rectangle::newi_sized(50, 50));
        let screen_bottom = Vector::y() * 100;
        let world_bottom = Vector::y() * 50;
        assert_eq!(viewport.project() * screen_bottom, world_bottom);
        assert_eq!(viewport.unproject() * world_bottom, screen_bottom);
    }
    
    #[test]
    fn custom_transform() {
        let rect = Rectangle::newi_sized(10, 10);
        let builder = ViewportBuilder::new(rect.size());
        let viewport = builder.get_viewport_transformed(rect, Transform::rotate(-90f32));
        let point = Vector::x() * 5;
        let expected = Vector::y() * 5;
        assert_eq!(viewport.project() * point, expected);
    }
}