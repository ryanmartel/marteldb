
pub type Buffer = [u8; 4096];

struct DiskPage {
    buffer: Buffer,
}


impl DiskPage {

    fn new() -> Self {
        DiskPage {
            buffer: [0;4096],
        }
    }

}
