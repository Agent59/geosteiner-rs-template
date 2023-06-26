use geosteiner_rs_template::geosteiner::{
    rs_safe_compute_esmt,
    Point,
};

fn main() { 
    let terms: [Point; 4] = [
        Point::new(0.0, 0.0),
        Point::new(0.0, 1.0),
        Point::new(1.0, 0.0),
        Point::new(1.0, 1.0),
    ];
    
    let esmt = rs_safe_compute_esmt::<4, 8>(4, &terms);

    //println!("{:?}", esmt.sps);
    //println!("{:?}", esmt.edges);
    println!("{:?}", esmt);
}
