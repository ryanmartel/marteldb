use super::tuple::Tuple;

struct MemPage {
    header: Header,
    item_ptrs: Vec<u32>,
    tuples: Vec<Tuple>
}


impl MemPage {




}

struct Header {
    lower: u32,
    upper: u32,
}

struct ItemPtrs {
    item_ptrs: Vec<u32>
}


impl Header {
    // fn copy_to(&self, arr: &mut Buffer) {
    //     let l = self.lower.to_le_bytes();
    //     let u = self.upper.to_le_bytes();
    //     arr[0..4].copy_from_slice(&l);
    //     arr[4..8].copy_from_slice(&u);
    // }
}

