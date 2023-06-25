pub use crate::{
    bindings::{squirreldatatypes::HSquirrelVM, unwraped::SquirrelFunctionsUnwraped},
    entry,
    high::{
        concommands::CCommandResult,
        engine::EngineData,
        northstar::{EngineLoadType, PluginData, ScriptVmType},
        squirrel::CSquirrelVMHandle,
    },
    mid::{engine::DLLPointer, squirrel::SQFUNCTIONS},
    plugin::Plugin,
};
pub use log;

// consider adding more stuff ^

/// puts a thread on sleep in milliseconds
pub fn wait(milliseconds: u64) {
    std::thread::sleep(std::time::Duration::from_millis(milliseconds))
}
