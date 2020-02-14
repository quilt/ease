use std::cell::Cell;
use std::mem::transmute;
extern crate byteorder;
use byteorder::{LittleEndian, WriteBytesExt};
use ewasm::{Execute, RootRuntime};
use std::convert::TryFrom;

pub struct ExecutionEnvironment {
    code: Vec<u8>,
    state: Vec<u8>,
    root: Cell<[u8; 32]>,
}

pub struct Simulation {
    execution_environments: Vec<ExecutionEnvironment>,
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
        let execution_environment = ExecutionEnvironment {
            code,
            state,
            root: Cell::new(root),
        };
        self.execution_environments.push(execution_environment);
        self.execution_environments.len() - 1
    }

    pub fn create_shard_block(&self, index: usize, txs: Vec<u8>, num_tx: u32) {
        let execution_environment = &self.execution_environments[index];
        let mut num_txs: Vec<u8> = Vec::new();
        num_txs.write_u32::<LittleEndian>(num_tx).unwrap();
        let data = &[
            num_txs,
            txs.to_owned(),
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
