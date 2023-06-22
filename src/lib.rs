//use cty; 

#[repr(C)]
pub struct gst_param_ptr {}

extern "C" {
    pub fn gst_open_geosteiner() -> i32;

    pub fn gst_close_geosteiner() -> i32;

    pub fn gst_esmt(
        nterms: i32,
        terms: i64,
        length: i64,
        nsps: i32,
        sps: i64,
        nedges: i32,
        edges: i32,
        status: i32,
        param: gst_param_ptr,
        );
}

pub fn rs_gst_open_geosteiner() -> i32 {
    unsafe { gst_open_geosteiner() }
}

pub fn rs_gst_close_geosteiner() -> i32 {
    unsafe { gst_close_geosteiner() }
}

//pub fn rs_gst_esmt()
