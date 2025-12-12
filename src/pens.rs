use skrifa::outline::OutlinePen;

#[derive(Default)]
pub struct PointCollector {
    scale: f32,
    points: Vec<[f32; 2]>,
}

impl PointCollector {
    pub fn new(scale: f32) -> Self {
        Self {
            scale,
            points: Vec::new(),
        }
    }

    pub fn into_points(self) -> Vec<[f32; 2]> {
        self.points
    }

    fn push(&mut self, x: f32, y: f32) {
        self.points.push([x * self.scale, y * self.scale]);
    }
}

impl OutlinePen for PointCollector {
    fn move_to(&mut self, x: f32, y: f32) {
        self.push(x, y);
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.push(x, y);
    }

    fn quad_to(&mut self, cx0: f32, cy0: f32, x: f32, y: f32) {
        self.push(cx0, cy0);
        self.push(x, y);
    }

    fn curve_to(&mut self, cx0: f32, cy0: f32, cx1: f32, cy1: f32, x: f32, y: f32) {
        self.push(cx0, cy0);
        self.push(cx1, cy1);
        self.push(x, y);
    }

    fn close(&mut self) {}
}

#[derive(Default)]
pub struct CommandCountPen {
    count: u32,
}

impl CommandCountPen {
    pub fn into_count(self) -> u32 {
        self.count
    }

    fn hit(&mut self) {
        self.count = self.count.saturating_add(1);
    }
}

impl OutlinePen for CommandCountPen {
    fn move_to(&mut self, _x: f32, _y: f32) {
        self.hit();
    }

    fn line_to(&mut self, _x: f32, _y: f32) {
        self.hit();
    }

    fn quad_to(&mut self, _cx0: f32, _cy0: f32, _x: f32, _y: f32) {
        self.hit();
    }

    fn curve_to(&mut self, _cx0: f32, _cy0: f32, _cx1: f32, _cy1: f32, _x: f32, _y: f32) {
        self.hit();
    }

    fn close(&mut self) {
        self.hit();
    }
}
