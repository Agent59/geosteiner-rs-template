use geosteiner_rust_api::{
    rs_gst_open_geosteiner,
    rs_gst_close_geosteiner,
    //gst_esmt,
};

fn main() {
    println!("test");

    if rs_gst_open_geosteiner() != 0 {
        println!("opened geosteiner");

        let terms = [0, 0, 0, 1, 1, 0, 1, 1];

        //unsafe { gst_esmt(4, terms, ); };
        
        rs_gst_close_geosteiner();
    }
}
