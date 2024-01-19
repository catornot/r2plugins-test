//! prelude for rrplug

pub use crate::{
    bindings::{
        squirrelclasstypes::ScriptContext, squirreldatatypes::HSquirrelVM,
        squirrelfunctions::SquirrelFunctions,
    },
    entry,
    high::{
        concommands::CCommandResult,
        convars::{ConVarRegister, ConVarStruct},
        engine::{EngineData, EngineGlobal, EngineToken},
        squirrel::{register_sq_functions, CSquirrelVMHandle},
        vector::Vector3,
    },
    mid::{
        engine::{DLLPointer, WhichDll},
        squirrel::SQFUNCTIONS,
    },
    plugin::Plugin,
};
pub use log;

// consider adding more stuff ^

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
