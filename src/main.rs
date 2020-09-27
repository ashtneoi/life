use core::char;
use core::convert::TryInto;

#[derive(Clone, Copy)]
pub struct LifeCell {
    lut_config: u16,
    a_config: u8,
    a_reg: u8,
    s_reg: u8,
    q_reg: u8,
}

impl LifeCell {
    pub fn step(&mut self, input: Output) {
        assert_eq!(self.a_config >> 1, 0);
        assert_eq!(self.a_reg >> 4, 0);
        assert_eq!(self.s_reg >> 4, 0);
        assert_eq!(self.q_reg >> 4, 0);

        if input.le == 1 || (self.a_reg & 0b0010) >> 1 == 1 {
            self.q_reg = self.s_reg;
        }
        if input.se == 1 || self.a_reg & 0b0001 == 1 {
            let lut_q = ((self.lut_config >> self.a_reg) & 1) as u8;
            self.s_reg = (self.s_reg >> 1) | (lut_q << 3);
        }
        self.a_reg = match self.a_config {
            0 => input.q,
            _ => self.a_reg.wrapping_add(1) & 0xF,
        };
    }

    pub fn output(&self) -> Output {
        Output {
            q: self.q_reg,
            se: (self.a_reg & 0b0100) >> 2,
            le: (self.a_reg & 0b1000) >> 3,
        }
    }

    pub fn as_char(&self) -> char {
        let x = self.s_reg as u32;
        match self.s_reg {
            0x0..=0x9 => '0' as u32 + x,
            0xA..=0xF => 'A' as u32 - 0xA + x,
            _ => unreachable!(),
        }.try_into().unwrap()
    }
}

#[derive(Clone, Copy)]
pub struct Output {
    q: u8,
    se: u8,
    le: u8,
}

const CELL_COUNT: usize = 64;

fn main() {
    let mut v = Vec::with_capacity(CELL_COUNT);
    for i in 0..CELL_COUNT {
        v.push(LifeCell {
            lut_config: 0x0184u16.wrapping_add(0x3B * i as u16),
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

        let v0_input = v[15].output();
        for i in (0..CELL_COUNT).rev() {
            let input = match i {
                0 => v0_input,
                _ => v[i - 1].output(),
            };
            v[i].step(input);
        }

        for i in 0..CELL_COUNT {
            print!("{}", v[i].as_char());
        }
        print!("\n");
        n = (n + 1) & 0xFF;
    }
}
