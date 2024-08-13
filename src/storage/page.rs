type Buffer = [u8; 4096];

struct DiskPage {
    buffer: Buffer,
}

impl 

impl DiskPage {

    fn new() -> Self {
        DiskPage {
            buffer: [0;4096],
        }
    }

    fn parse_page() -> MemPage {
        MemPage::new()
    }
}

struct MemPage {
    header: Header,

}

impl MemPage {

    fn new() -> Self {
        MemPage {
            header: Header{
                lower: 0,
                upper: 0
            }
        }
    }

    fn serialize(&self) -> DiskPage {
        let mut dp = DiskPage::new();
        self.header.copy_to(&mut dp.buffer);
        dp
    }
}

struct Header {
    lower: u32,
    upper: u32,
}

impl Header {
    fn copy_to(&self, arr: &mut Buffer) {
        let l = self.lower.to_le_bytes();
        let u = self.upper.to_le_bytes();
        arr[0..4].copy_from_slice(&l);
        arr[4..8].copy_from_slice(&u);
    }
}

struct ItemPointers {
    items: Vec<u32>
}

pub fn test_buffer() {
    let ph = Header {
        lower: u32::MAX,
        upper: 40100,
    };
    let mp = MemPage {
        header: ph
    };
    let dp = mp.serialize();
    println!("{:?}", dp.buffer);
}
