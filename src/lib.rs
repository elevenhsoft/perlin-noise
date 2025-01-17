const PERMUTATION: [i32; 256] = [
    151, 160, 137, 91, 90, 15, 131, 13, 201, 95, 96, 53, 194, 233, 7, 225, 140, 36, 103, 30, 69,
    142, 8, 99, 37, 240, 21, 10, 23, 190, 6, 148, 247, 120, 234, 75, 0, 26, 197, 62, 94, 252, 219,
    203, 117, 35, 11, 32, 57, 177, 33, 88, 237, 149, 56, 87, 174, 20, 125, 136, 171, 168, 68, 175,
    74, 165, 71, 134, 139, 48, 27, 166, 77, 146, 158, 231, 83, 111, 229, 122, 60, 211, 133, 230,
    220, 105, 92, 41, 55, 46, 245, 40, 244, 102, 143, 54, 65, 25, 63, 161, 1, 216, 80, 73, 209, 76,
    132, 187, 208, 89, 18, 169, 200, 196, 135, 130, 116, 188, 159, 86, 164, 100, 109, 198, 173,
    186, 3, 64, 52, 217, 226, 250, 124, 123, 5, 202, 38, 147, 118, 126, 255, 82, 85, 212, 207, 206,
    59, 227, 47, 16, 58, 17, 182, 189, 28, 42, 223, 183, 170, 213, 119, 248, 152, 2, 44, 154, 163,
    70, 221, 153, 101, 155, 167, 43, 172, 9, 129, 22, 39, 253, 19, 98, 108, 110, 79, 113, 224, 232,
    178, 185, 112, 104, 218, 246, 97, 228, 251, 34, 242, 193, 238, 210, 144, 12, 191, 179, 162,
    241, 81, 51, 145, 235, 249, 14, 239, 107, 49, 192, 214, 31, 181, 199, 106, 157, 184, 84, 204,
    176, 115, 121, 50, 45, 127, 4, 150, 254, 138, 236, 205, 93, 222, 114, 67, 29, 24, 72, 243, 141,
    128, 195, 78, 66, 215, 61, 156, 180,
];

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn grad(hash: i32, x: f64, y: f64, z: f64) -> f64 {
    match hash & 0xF {
        0x0 => x + y,
        0x1 => -x + y,
        0x2 => x - y,
        0x3 => -x - y,
        0x4 => x + z,
        0x5 => -x + z,
        0x6 => x - z,
        0x7 => -x - z,
        0x8 => y + z,
        0x9 => -y + z,
        0xA => y - z,
        0xB => -y - z,
        0xC => y + x,
        0xD => -y + z,
        0xE => y - x,
        0xF => -y - z,
        _ => 0.0,
    }
}

fn lerp(a: f64, b: f64, x: f64) -> f64 {
    a + x * (b - a)
}

pub struct Perlin {
    repeat: i32,
    p: [i32; 512],
}

impl Perlin {
    pub fn new(repeat: i32) -> Self {
        let mut p: [i32; 512] = [0; 512];

        for x in 0..512 {
            p[x] = PERMUTATION[x % 256];
        }

        Self { repeat, p }
    }

    fn inc(&mut self, mut num: i32) -> i32 {
        num += 1;

        if self.repeat > 0 {
            num %= self.repeat;
        }

        num
    }

    pub fn noise(&mut self, mut x: f64, mut y: f64, mut z: f64) -> f64 {
        if self.repeat > 0 {
            x %= self.repeat as f64;
            y %= self.repeat as f64;
            z %= self.repeat as f64;
        };

        let xi: i32 = x.floor() as i32 & 255;
        let yi: i32 = y.floor() as i32 & 255;
        let zi: i32 = z.floor() as i32 & 255;
        let xf: f64 = x - x.floor();
        let yf: f64 = y - y.floor();
        let zf: f64 = z - z.floor();

        let u: f64 = fade(xf);
        let v: f64 = fade(yf);
        let w: f64 = fade(zf);

        let aaa = self.p[(self.p[(self.p[xi as usize] + yi) as usize] + zi) as usize];
        let aba = self.p[(self.p[(self.p[xi as usize] + self.inc(yi)) as usize] + zi) as usize];
        let aab = self.p[(self.p[(self.p[xi as usize] + yi) as usize] + self.inc(zi)) as usize];
        let abb =
            self.p[(self.p[(self.p[xi as usize] + self.inc(yi)) as usize] + self.inc(zi)) as usize];
        let baa = self.p[(self.p[(self.p[self.inc(xi) as usize] + yi) as usize] + zi) as usize];
        let bba =
            self.p[(self.p[(self.p[self.inc(xi) as usize] + self.inc(yi)) as usize] + zi) as usize];
        let bab =
            self.p[(self.p[(self.p[self.inc(xi) as usize] + yi) as usize] + self.inc(zi)) as usize];
        let bbb = self.p[(self.p[(self.p[self.inc(xi) as usize] + self.inc(yi)) as usize]
            + self.inc(zi)) as usize];

        let x1 = lerp(grad(aaa, xf, yf, zf), grad(baa, xf - 1.0, yf, zf), u);
        let x2 = lerp(
            grad(aba, xf, yf - 1.0, zf),
            grad(bba, xf - 1.0, yf - 1.0, zf),
            u,
        );
        let y1 = lerp(x1, x2, v);

        let x1 = lerp(
            grad(aab, xf, yf, zf - 1.0),
            grad(bab, xf - 1.0, yf, zf - 1.0),
            u,
        );
        let x2 = lerp(
            grad(abb, xf, yf - 1.0, zf - 1.0),
            grad(bbb, xf - 1.0, yf - 1.0, zf - 1.0),
            u,
        );
        let y2 = lerp(x1, x2, v);

        (lerp(y1, y2, w) + 1.0) / 2.0
    }

    pub fn octave(&mut self, x: f64, y: f64, z: f64, octaves: usize, persistence: f64) -> f64 {
        let mut total = 0.0;
        let mut frequency = 1.0;
        let mut amplitude = 1.0;
        let mut max_value = 0.0;

        for _ in 0..octaves {
            total += self.noise(x * frequency, y * frequency, z * frequency) * amplitude;
            max_value += amplitude;
            amplitude *= persistence;
            frequency *= 2.0;
        }

        total / max_value
    }
}
