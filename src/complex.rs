use std::ops;
use std::fmt;
use std::f64;
use std::f64::consts::PI;

#[derive(Clone, Copy)]
pub struct Cpx {
    pub r: f64,
    pub c: f64,
}

impl Cpx {
    pub fn new<T: Into<f64>>(r: T, c: T) -> Cpx {
        Cpx { r: r.into(), c: c.into() }
    }
    pub fn abs(&self) -> f64 {
        (self.r * self.r + self.c * self.c).sqrt()
    }
    pub fn phi(&self) -> f64 {
        if self.r == 0.0 && self.c == 0.0 {
            0.0
        } else if self.r < 0.0 && self.c == 0.0 {
            PI
        }else {
            let a = 2.0 * (self.c / (self.abs() + self.r)).atan();
            if a >= 0.0 { a } else { 2.0 * PI + a }
        }
        //self.r.atan2(self.c)
    }
    
    pub fn sin(n: Cpx) -> Cpx {
        Cpx::new(n.r.sin() * n.c.cosh(),
        n.r.cos() * n.c.sinh())
    }
    pub fn tan(n: Cpx) -> Option<Cpx> {
        Cpx::new((2.0 * n.r).sin(), (2.0 * n.c).sinh()) /
            Cpx::new((2.0 * n.r).cos(), (2.0 * n.c).cosh())
    }
}

// -- ADDITION --------------------------
impl ops::Add<Cpx> for Cpx {
    type Output = Cpx;
    fn add(self, _rhs: Cpx) -> Cpx {
        Cpx { r: self.r + _rhs.r, 
              c: self.c + _rhs.c }
    }
}

// -- SUBTRACTIONS ----------------------
impl ops::Sub<Cpx> for Cpx {
    type Output = Cpx;
    fn sub(self, _rhs: Cpx) -> Cpx {
        Cpx { r: self.r - _rhs.r, 
              c: self.c - _rhs.c }
    }
}

// -- MUTLIPLICATION --------------------
impl ops::Mul<Cpx> for Cpx {
    type Output = Cpx;
    fn mul(self, _rhs: Cpx) -> Cpx {
        Cpx { r: self.r * _rhs.r - self.c * _rhs.c, 
              c: self.r * _rhs.c + _rhs.r * self.c }
    }
}

// -- DIVISION --------------------------
impl ops::Div<Cpx> for Cpx {
    type Output = Option<Cpx>;
    fn div(self, _rhs: Cpx) -> Option<Cpx> {
        let cs : f64 = _rhs.r * _rhs.r + _rhs.c * _rhs.c;
        if cs == 0.0 { 
            None 
        } else {
            Some(Cpx { r: (self.r * _rhs.r + self.c * _rhs.c) / cs, 
                       c: (self.c * _rhs.r - self.r * _rhs.c) / cs })
        }
    }
}


impl fmt::Display for Cpx {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.c >= 0.0 {
            write!(f, "{}+{}i", self.r, self.c)
        } else {
            write!(f, "{}{}i", self.r, self.c)
        }
    }
}
