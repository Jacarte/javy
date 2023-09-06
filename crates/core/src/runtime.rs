use anyhow::Result;
use javy::{Config, Runtime};
use javy_apis::{APIConfig, LogStream, RuntimeExt};


#[derive(serde::Deserialize, Debug)]
pub struct OfTwo {
    pub WHITELIST: Vec<String>,
    pub BLACKLIST: Vec<String>
}

#[derive(serde::Deserialize, Debug)]
pub struct FilePermissions {
    pub READ: OfTwo,
    pub WRITE: OfTwo
}

pub(crate) fn new_runtime() -> Result<Runtime> {
    let mut api_config = APIConfig::default();
    api_config.log_stream(LogStream::StdErr);
    
    Runtime::new_with_apis(Config::default(), api_config)
}


pub(crate) fn new_runtime_with_file_permissions(
    permissions: FilePermissions
) -> Result<Runtime> {
    let mut api_config = APIConfig::default();
    api_config.log_stream(LogStream::StdErr);

    let mut fsconfig = javy_apis::fs::FSConfig::default();
    eprintln!("File permissions: {:?}", permissions);
    
    for file in permissions.READ.WHITELIST {
        fsconfig.add_to_read_whitelist(&file);
    }
    for file in permissions.READ.BLACKLIST {
        fsconfig.add_to_read_blacklist(&file);
    }
    for file in permissions.WRITE.WHITELIST {
        fsconfig.add_to_write_whitelist(&file);
    }
    for file in permissions.WRITE.BLACKLIST {
        fsconfig.add_to_write_blacklist(&file);
    }
    
    api_config.fs = fsconfig;
    
    Runtime::new_with_apis(Config::default(), api_config)
}
