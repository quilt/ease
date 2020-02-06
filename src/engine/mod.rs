pub struct Engine {}

impl Engine {
    pub fn new() -> Engine {
        Engine {}
    }

    /// Deploy EE code onchain
    pub fn deploy(&self, code: Vec<u8>, state: Vec<u8>) {}

    /// Send tx to be run onchain
    pub fn run(&self, tx: Vec<u8>) {}

    /// Get onchain state root
    pub fn get_root(&self) {}
}
