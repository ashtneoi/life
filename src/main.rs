use core::char;
use core::convert::TryInto;
use nix::unistd::isatty;
use rand::random;
use std::io;
use std::os::unix::prelude::*;

fn format_hex_char(x: u8) -> char {
    let x = x as u32;
    match x {
        0x0..=0x9 => '0' as u32 + x,
        0xA..=0xF => 'A' as u32 - 0xA + x,
        _ => unreachable!(),
    }.try_into().unwrap()
}

#[derive(Clone, Copy)]
pub struct LifeCell {
    light: u8, // 4 bits
    lut_config: u16, // 16 bits
    a_config: u8, // 4 bits
    a_reg: u8, // 4 bits
    s_reg: u8, // 4 bits
    q_reg: u8, // 4 bits
}

impl LifeCell {
    pub fn poke(&mut self) {
        self.s_reg = random::<u8>() & 0xF;
    }

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

        self.lut_config = self.lut_config.wrapping_add(self.light as u16);
    }

    pub fn output(&self) -> Output {
        Output {
            q: self.q_reg,
            se: (self.a_reg & 0b0100) >> 2,
            le: (self.a_reg & 0b1000) >> 3,
        }
    }

    pub fn as_u4(&self) -> u8 {
        self.s_reg
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
    let stdout_is_tty = isatty(
        io::stdout().as_raw_fd()
    ).unwrap();

    let mut v = Vec::with_capacity(CELL_COUNT);
    for i in 0..CELL_COUNT {
        v.push(LifeCell {
            light: (i & 0xF) as u8,
            lut_config: (i as u16).wrapping_mul(0x010F),
            a_config: 0,
            a_reg: 0xE,
            s_reg: 0x6,
            q_reg: 0x5,
        });
    }

    let mut v = v.into_boxed_slice();
    //v[0].poke();
    v[0].s_reg = 0x0;

    let mut n = 0;

    loop {
        //if n == 0 {
            //println!("!");
            //v[0].poke();
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
            let x = v[i].as_u4();
            if stdout_is_tty && x > 0x8 {
                print!("\x1B[7m");
            }
            print!("{}", format_hex_char(x));
            if stdout_is_tty && x > 0x8 {
                print!("\x1B[0m");
            }
        }
        print!("\n");
        n = (n + 1) & 0xFF;
    }
}
