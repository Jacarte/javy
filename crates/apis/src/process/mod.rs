use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct Process;

impl JSApiSet for Process {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        let mut javy_object = global.get_property("process")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("process", javy_object)?;
        }


        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::{random::Random, APIConfig, JSApiSet};
    use anyhow::Result;
    use javy::Runtime;

    #[test]
    fn test_random() -> Result<()> {
        let runtime = Runtime::default();
        Random.register(&runtime, &APIConfig::default())?;
        let ctx = runtime.context();
        // Return time since 1970 in microseconds
        ctx.eval_global("test.js", "result = __date_clock()")?;
        let result = ctx.global_object()?.get_property("result")?;
        println!("{:?}", result);
        Ok(())
    }
}
