#![allow(clippy::clone_on_ref_ptr)]
#![allow(clippy::missing_const_for_fn)]

pub mod highway {
    tonic::include_proto!("highway");
}

pub mod metrics {
    tonic::include_proto!("metrics");
}
