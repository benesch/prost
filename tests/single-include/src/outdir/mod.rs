pub mod outdir {
    include!("outdir.rs");
}
#[allow(dead_code)]
pub const FILE_DESCRIPTOR_SET_DATA: &[u8] = include_bytes!("src/outdir/file_descriptor_set.pb");
