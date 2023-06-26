use geosteiner_rs_api::{
    rs_safe_compute_esmt,
};

fn main() {
    let terms: [f64; 8] = [0.0, 0.0,
                        0.0, 1.0,
                        1.0, 0.0,
                        1.0, 1.0];
    
    let esmt = rs_safe_compute_esmt(4, &terms);

    println!("{:?}", esmt.sps);
}
