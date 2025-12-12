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

#[derive(Default)]
pub struct CommandBreakdownPen {
    move_to: u64,
    line_to: u64,
    quad_to: u64,
    curve_to: u64,
    close: u64,
}

impl CommandBreakdownPen {
    pub fn counts(&self) -> [u64; 5] {
        [
            self.move_to,
            self.line_to,
            self.quad_to,
            self.curve_to,
            self.close,
        ]
    }
}

impl OutlinePen for CommandBreakdownPen {
    fn move_to(&mut self, _x: f32, _y: f32) {
        self.move_to = self.move_to.saturating_add(1);
    }

    fn line_to(&mut self, _x: f32, _y: f32) {
        self.line_to = self.line_to.saturating_add(1);
    }

    fn quad_to(&mut self, _cx0: f32, _cy0: f32, _x: f32, _y: f32) {
        self.quad_to = self.quad_to.saturating_add(1);
    }

    fn curve_to(&mut self, _cx0: f32, _cy0: f32, _cx1: f32, _cy1: f32, _x: f32, _y: f32) {
        self.curve_to = self.curve_to.saturating_add(1);
    }

    fn close(&mut self) {
        self.close = self.close.saturating_add(1);
    }
}
