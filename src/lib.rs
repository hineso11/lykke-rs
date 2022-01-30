include!(concat!(env!("OUT_DIR"), "/hft.rs"));

pub mod common {
    include!(concat!(env!("OUT_DIR"), "/common.rs"));
    include!(concat!(env!("OUT_DIR"), "/hft.common.rs"));
}