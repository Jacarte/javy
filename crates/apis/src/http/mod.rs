use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) use config::HTTPConfig;
mod config;

pub(super) struct HTTP;

impl JSApiSet for HTTP {
    
    fn register(&self, runtime: &Runtime, config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        let mut javy_object = global.get_property("Node")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("Node", javy_object)?;
        }
        
        let configcp = config.http.clone();

        global.set_property(
            "__request",
            context.wrap_callback(move |_, _this_arg, args| {
                let [endpoint, data, headers, method, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };

                todo!("Need to be implemented");
                Ok(0.into())
            })?,
        )?;
        context.eval_global("http.js", include_str!("http.js"))?;
        Ok(())
    }
}





#[cfg(test)]
mod tests {
    use crate::{http::HTTP, APIConfig, JSApiSet};
    use anyhow::Result;
    use javy::Runtime;

}