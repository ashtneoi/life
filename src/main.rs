#[derive(Clone)]
struct LifeCell {
    lut_config: u16,
    a_config: u8,
    a_reg: u8,
    s_reg: u8,
    q_reg: u8,
}

impl LifeCell {
    pub fn step(&mut self, a_in: u8, se_in: u8, le_in: u8) {
        assert_eq!(self.a_config >> 1, 0);
        assert_eq!(self.a_reg >> 4, 0);
        assert_eq!(self.s_reg >> 4, 0);
        assert_eq!(self.q_reg >> 4, 0);

        if le_in == 1 || (self.a_reg & 0b0010) >> 1 == 1 {
            self.q_reg = self.s_reg;
        }
        if se_in == 1 || self.a_reg & 0b0001 == 1 {
            let lut_q = ((self.lut_config >> self.a_reg) & 1) as u8;
            self.s_reg = (self.s_reg >> 1) | (lut_q << 3);
        }
        self.a_reg = match self.a_config {
            0 => a_in,
            _ => self.a_reg.wrapping_add(1) & 0xF,
        };
    }

    pub fn q(&self) -> u8 {
        self.q_reg
    }

    pub fn se_out(&self) -> u8 {
        (self.a_reg & 0b0100) >> 2
    }

    pub fn le_out(&self) -> u8 {
        (self.a_reg & 0b1000) >> 3
    }
}

const CELL_COUNT: usize = 64;

fn main() {
    let mut v = Vec::with_capacity(CELL_COUNT);
    for i in 0..CELL_COUNT {
        v.push(LifeCell {
            lut_config: 0xF300,
            a_config: 0,
            a_reg: (i & 0xF) as u8,
            s_reg: 0x6,
            q_reg: 0x5,
        });
    }

    let mut v = v.into_boxed_slice();

    let mut n = 0;

    loop {
        //if n == 0 {
            //println!("!");
            //v[0].s_reg = 0x3;
        //}

        let v0_a_in = v[CELL_COUNT - 1].q();
        let v0_se_in = v[CELL_COUNT - 1].se_out();
        let v0_le_in = v[CELL_COUNT - 1].le_out();
        for i in (0..CELL_COUNT).rev() {
            let a_in;
            let se_in;
            let le_in;
            if i > 0 {
                a_in = v[i - 1].q();
                se_in = v[i - 1].se_out();
                le_in = v[i - 1].le_out();
            } else {
                a_in = (n & 0xF) as u8;
                //a_in = v0_a_in;
                se_in = v0_se_in;
                //le_in = v0_le_in;
                le_in = 0;
            }
            v[i].step(a_in, se_in, le_in);
        }

        for i in 0..CELL_COUNT {
            print!("{:1X}", v[i].s_reg);
        }
        print!("\n");
        n = (n + 1) & 0xFF;
    }
}
