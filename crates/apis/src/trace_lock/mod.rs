use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct NodeRed;

extern "Rust" {

    /// Lock the traces
    pub fn lock();

    /// Unlock the traces
    pub fn unlock();
}

impl JSApiSet for NodeRed {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        let mut javy_object = global.get_property("Trace")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("Trace", javy_object)?;
        }

        global.set_property(
            "__trace_lock",
            context.wrap_callback(|_, _this_arg, args| {
                unsafe { lock() };
            })?,
        )?;

        global.set_property("__trace_unlock", context.wrap_callback(|_, _this_arg, args|{
                unsafe { unlock() };
            })?,
        )?;

        context.eval_global("trace_lock.js", include_str!("trace_lock.js"))?;
        Ok(())
    }
}
