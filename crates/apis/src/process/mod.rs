use anyhow::Result;
use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct Process;

impl JSApiSet for Process {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::{process::Process, APIConfig, JSApiSet};
    use anyhow::Result;
    use javy::Runtime;

    #[test]
    fn test_date() -> Result<()> {
        let runtime = Runtime::default();
        Process.register(&runtime, &APIConfig::default())?;
        let ctx = runtime.context();
        // Return time since 1970 in microseconds
        ctx.eval_global("test.js", "result = __date_clock()")?;
        let result = ctx.global_object()?.get_property("result")?;
        println!("{:?}", result);
        Ok(())
    }

}
