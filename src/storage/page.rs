type Buffer = [u8; 4096];

struct Page {
    buf: Buffer
}



impl Page {

    fn new() -> Self {
        Page {
            buf: [0;4096]
        }
   }


}




