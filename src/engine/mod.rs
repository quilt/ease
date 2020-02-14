mod simulation;
use simulation::Simulation;

pub struct Engine {
    simulation: Simulation,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            simulation: Simulation::new(),
        }
    }

    /// Deploy EE code onchain
    pub fn deploy(&mut self, code: Vec<u8>, state: Vec<u8>, root: [u8; 32]) -> usize {
        self.simulation
            .create_execution_environment(code, state, root)
    }

    /// Send tx(s) to be run onchain
    pub fn run(&self, index: usize, txs: Vec<u8>, num_tx: u32) {
        self.simulation.create_shard_block(index, txs, num_tx);
    }

    /// Get onchain state root
    pub fn get_root(&self, index: usize) -> [u8; 32] {
        self.simulation.get_execution_environment_root(index)
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}
