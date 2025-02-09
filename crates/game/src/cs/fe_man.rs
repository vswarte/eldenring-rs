#[repr(C)]
#[derive(Debug)]
#[dlrf::singleton("CSFeMan")]
pub struct CSFeMan {
    vftable: usize,
}
