pub struct Random {
    x: u64,
    y: u64,
    z: u64,
    w: u64,
}

impl Random {
    pub fn new() -> Self {
        let mut entity = Self { x: 123456789, y: 362436069, z: 521288629, w: 88675123 };
        for _ in 0..8 { entity.next(); };
        entity
    }

    fn next(&mut self) -> u64 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (t ^ (t >> 8));
        self.w
    }

    pub fn next_bound(&mut self, min: u64, max: u64) -> u64 {
        min + self.next() % (max - min)
    }

    pub fn next_1byte(&mut self) -> u64 {
        self.next_bound(0, 256)
    }

    pub fn next_f(&mut self) -> f64 {
        self.next() as f64 / u64::MAX as f64
    }

    pub fn next_f_bound(&mut self, min: f64, max: f64) -> f64 {
        min + (max - min) * self.next_f()
    }
}