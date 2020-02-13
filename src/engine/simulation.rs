use std::cell::Cell;
use std::mem::transmute;
extern crate byteorder;
use byteorder::{ByteOrder, LittleEndian, WriteBytesExt};
use ewasm::{Execute, RootRuntime};
use std::convert::TryFrom;

pub struct Execution_Environment {
    code: Vec<u8>,
    state: Vec<u8>,
    root: Cell<[u8; 32]>,
}

pub struct Simulation {
    execution_environments: Vec<Execution_Environment>,
}

impl Simulation {
    pub fn new() -> Simulation {
        Simulation {
            execution_environments: vec![],
        }
    }

    pub fn create_execution_environment(
        &mut self,
        code: Vec<u8>,
        state: Vec<u8>,
        root: [u8; 32],
    ) -> usize {
        let execution_environment = Execution_Environment {
            code: code,
            state: state,
            root: Cell::new(root),
        };
        self.execution_environments.push(execution_environment);
        let index = self.execution_environments.len() - 1;
        return index;
    }

    pub fn create_shard_block(&self, index: usize, txs: Vec<Vec<u8>>) {
        let execution_environment = &self.execution_environments[index];
        let mut num_tx: Vec<u8> = Vec::new();
        num_tx
            .write_u32::<LittleEndian>(u32::try_from(txs.len()).unwrap())
            .unwrap();
        let data = &[
            num_tx.to_vec(),
            txs[0].to_owned(),
            execution_environment.state.to_owned(),
        ]
        .concat();
        println!("pre root: {:?}", execution_environment.root.get());
        let mut runtime = RootRuntime::new(
            &execution_environment.code,
            data,
            execution_environment.root.get(),
        );

        // set new root
        let root = runtime.execute();
        println!("post root: {:?}", root);
        execution_environment.root.set(root);
        // set new state
        let bytes: [u8; 4] = unsafe { transmute(1u32.to_be()) };
        execution_environment
            .state
            .to_owned()
            .append(&mut bytes.to_vec());
    }

    pub fn get_execution_environment_root(&self, index: usize) -> [u8; 32] {
        let execution_environment = &self.execution_environments[index];
        execution_environment.root.get()
    }
}
