pub struct Position {
    x: f32,
    y: f32,
}

pub struct Size {
    width: f32,
    height: f32,
}

pub struct Rect {
    position: Position,
    size: Size,
}

pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Rect {
    #[inline]
    pub fn x_max(&self) -> f32 {
        self.position.x + self.size.width
    }

    #[inline]
    pub fn y_max(&self) -> f32 {
        self.position.y + self.size.height
    }

    #[inline]
    pub fn contains(&self, point: Position) -> bool {
        let position = &self.position;
        let size = &self.size;

        if point.x >= position.x
            && point.y >= position.y
            && point.x <= self.x_max()
            && point.y <= self.y_max()
        {
            return true;
        }
        false
    }
}
