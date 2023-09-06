use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct Globals;

impl JSApiSet for Globals {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        // Define some patched globals

        let mut javy_object = global.get_property("process")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("process", javy_object)?;
        }

        context.eval_global("globals.js", include_str!("globals.js"))?;
        Ok(())
    }
}
