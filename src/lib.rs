use std::os::raw::{
    c_int,
    c_double,
};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    x: f64,
    y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self { Point { x, y } }
}

/// A C struct that contains information on a euclidian steiner tree.
/// Because it is returned from a C function, it is very problematic to use directly.
/// Please us the rust representation `ESMT` instead.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct C_ESMT {
    pub length: c_double,
    pub nsps: c_int,
    pub sps: *const c_double,
}

/// Contains information on a euclidian steiner tree.
pub struct ESMT {
    pub sps: Vec<Point>,
}

impl ESMT {

    pub fn from_c_esmt(c_esmt: C_ESMT) -> Self {
        let v = unsafe { std::slice::from_raw_parts(c_esmt.sps, (c_esmt.nsps*2) as usize) }.to_vec();

        let mut sps = Vec::new();

        for i in 0..c_esmt.nsps {
            sps.push(Point::new(v[2*i as usize], v[(2*i+1) as usize]));
        }

        Self { sps }
    }
}

extern "C" {
    pub fn rs_compute_esmt (nterms: c_int, terms: *const c_double) -> C_ESMT;
}

pub fn rs_safe_compute_esmt<const L: usize>(nterms: i32, terms: &[f64; L]) -> ESMT {
    let c_esmt = unsafe { rs_compute_esmt(nterms, &terms[0]) };
    ESMT::from_c_esmt(c_esmt)
}
