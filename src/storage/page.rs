type Block = [u8; 4096];

struct PageHeader {
    lower: u32,
    upper: u32,
}

impl PageHeader {
    fn copy_to(&self, arr: &mut Block) {
        let l = self.lower.to_le_bytes();
        let u = self.upper.to_le_bytes();
        arr[0..4].copy_from_slice(&l);
        arr[4..8].copy_from_slice(&u);
    }
}

pub fn test_buffer() {
    let mut b: Block = [0;4096];
    let ph = PageHeader {
        lower: u32::MAX,
        upper: 40100,
    };
    ph.copy_to(&mut b);
    println!("{:?}", b);
}
