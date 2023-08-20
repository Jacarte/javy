//! fs tiny implementation based on a permission layer.
//! permissions are based on 4 lists: whilelist read, whitelist write, blacklist read, blacklist write
//! the lists contain glob patterns.
//! The idea is to use Node.FS as a wrapper around fs
//! ```js
//! let fs = Node.FS;
//! ```
use anyhow::Result;
use std::io::{Read, Write};
use std::io::Seek;
use javy::{
    Runtime, 
    quickjs::{JSContextRef, JSError, JSValue, JSValueRef},
};
use std::collections::HashMap;
use crate::APIConfig;

use crate::JSApiSet;

pub(super) use config::FSConfig;
mod config;

pub(super) struct FS;

#[derive(Debug)]
pub enum FileFlag{
    Append, // a
    AppendCreateIfNotExist, // a+
    AppendButFailIfExist, // ax
    Write, // w
    WriteButFailIfExist, // wx
    ReadWrite, // w+
    ReadWriteButFailIfExist, // wx+
    Read,
}

// According to documentation
/*

    'a': Open file for appending. The file is created if it does not exist.
    'a+': Open file for reading and appending. The file is created if it does not exist.
    'as': Open file for appending in synchronous mode. The file is created if it does not exist.
    'w': Open file for writing. The file is created (if it does not exist) or truncated (if it exists).
    'wx': Like 'w' but fails if the path exists.
    'w+': Open file for reading and writing. The file is created (if it does not exist) or truncated (if it exists).
    'wx+': Like 'w+' but fails if the path exists.

*/

impl TryFrom<String> for FileFlag {
    type Error = anyhow::Error;
    fn try_from(value: String) -> Result<Self> {
        match value.as_str() {
            "r" => Ok(FileFlag::Read),
            "a" => Ok(FileFlag::Append),
            "a+" => Ok(FileFlag::AppendCreateIfNotExist),
            "ax" => Ok(FileFlag::AppendButFailIfExist),
            "ax+" => Ok(FileFlag::AppendButFailIfExist),
            "w" => Ok(FileFlag::Write),
            "wx" => Ok(FileFlag::WriteButFailIfExist),
            "w+" => Ok(FileFlag::ReadWrite),
            "wx+" => Ok(FileFlag::ReadWriteButFailIfExist),
            _ => anyhow::bail!("Invalid flag {}", value)
        }
    }
}


impl JSApiSet for FS {
    fn register(&self, runtime: &Runtime, config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        let mut javy_object = global.get_property("Node")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("Node", javy_object)?;
        }


        let configcp = config.fs.clone();
        global.set_property(
            "__writeFileSync",
            context.wrap_callback(move |_, _this_arg, args| {
                let [filename, data, offset, flag, encoding, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };

                let filename: String = filename.try_into()?;
                if configcp.can_write(&filename){
                    let data = data.as_bytes_mut()?;

                    let flag: String = flag.try_into()?;
                    let flag: FileFlag = flag.try_into()?;
                    let offset: i32 = offset.try_into()?;
                    let encoding: String = encoding.try_into()?;
                    eprintln!("Flag {:?}", flag);

                    // match of options is string or object
                    // Get the file
                    // Create if append is false
                    let mut fd = match flag {
                        FileFlag::Read => std::fs::OpenOptions::new().read(true).open(filename)?,
                        FileFlag::Append => std::fs::OpenOptions::new().append(true).open(filename)?,
                        FileFlag::AppendCreateIfNotExist => std::fs::OpenOptions::new().append(true).create(true).open(filename)?,
                        FileFlag::AppendButFailIfExist => std::fs::OpenOptions::new().append(true).create_new(true).open(filename)?,
                        FileFlag::Write => std::fs::OpenOptions::new().write(true).open(filename)?,
                        FileFlag::WriteButFailIfExist => std::fs::OpenOptions::new().write(true).create_new(true).open(filename)?,
                        FileFlag::ReadWrite => std::fs::OpenOptions::new().read(true).write(true).open(filename)?,
                        FileFlag::ReadWriteButFailIfExist => std::fs::OpenOptions::new().read(true).write(true).create_new(true).open(filename)?,
                    };
                    // TODO set the writting encoding
                    // Set the offset to write
                    fd.seek(std::io::SeekFrom::Start(offset as u64))?;

                    let n = fd.write(&data)?;
                    // let n = fd.write(&data)?;
                    fd.flush()?;

                    Ok(n.into())

                } else {
                    // Just emit a message, be quiet
                    eprintln!("[WASM] warning {} has no write permissions", filename);
                    Ok(0.into())
                }
                
            })?,
        )?;

        let configcp = config.fs.clone();
        global.set_property(
            "__readFileSync",
            context.wrap_callback(move |_, _this_arg, args| {
                let [filename, data, offset, flag, encoding, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };

                let filename: String = filename.try_into()?;
                if configcp.can_read(&filename){
                    let mut data = data.as_bytes_mut()?;
                    let offset: i32 = offset.try_into()?;
                    let flag: String = flag.try_into()?;
                    let flag: FileFlag = flag.try_into()?;
                    let encoding: String = encoding.try_into()?;
                    // match of options is string or object
                    // Get the file
                    // Create if append is false
                    let mut fd = match flag {
                        FileFlag::Read => std::fs::OpenOptions::new().read(true).open(filename)?,
                        FileFlag::Append => std::fs::OpenOptions::new().append(true).open(filename)?,
                        FileFlag::AppendCreateIfNotExist => std::fs::OpenOptions::new().append(true).create(true).open(filename)?,
                        FileFlag::AppendButFailIfExist => std::fs::OpenOptions::new().append(true).create_new(true).open(filename)?,
                        FileFlag::Write => std::fs::OpenOptions::new().write(true).open(filename)?,
                        FileFlag::WriteButFailIfExist => std::fs::OpenOptions::new().write(true).create_new(true).open(filename)?,
                        FileFlag::ReadWrite => std::fs::OpenOptions::new().read(true).write(true).open(filename)?,
                        FileFlag::ReadWriteButFailIfExist => std::fs::OpenOptions::new().read(true).write(true).create_new(true).open(filename)?,
                    };
                    fd.seek(std::io::SeekFrom::Start(offset as u64))?;

                    let n = fd.read(&mut data)?;

                    Ok(n.into())

                } else {
                    // Just emit a message, be quiet
                    eprintln!("[WASM] warning {} has no read permissions", filename);
                    Ok(0.into())
                }
            })?,
        )?;

        let configcp = config.fs.clone();
        global.set_property(
            "__statSync",
            context.wrap_callback(move |_, _this_arg, args| {
                let [filename, flag, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };

                let filename: String = filename.try_into()?;

                if configcp.can_read(&filename){
                    let flag: String = flag.try_into()?;
                    let flag: FileFlag = flag.try_into()?;
                    // match of options is string or object
                    // Get the file
                    // Create if append is false
                    let mut fd = match flag {
                        FileFlag::Read => std::fs::OpenOptions::new().read(true).open(filename)?,
                        FileFlag::Append => std::fs::OpenOptions::new().append(true).open(filename)?,
                        FileFlag::AppendCreateIfNotExist => std::fs::OpenOptions::new().append(true).create(true).open(filename)?,
                        FileFlag::AppendButFailIfExist => std::fs::OpenOptions::new().append(true).create_new(true).open(filename)?,
                        FileFlag::Write => std::fs::OpenOptions::new().write(true).open(filename)?,
                        FileFlag::WriteButFailIfExist => std::fs::OpenOptions::new().write(true).create_new(true).open(filename)?,
                        FileFlag::ReadWrite => std::fs::OpenOptions::new().read(true).write(true).open(filename)?,
                        FileFlag::ReadWriteButFailIfExist => std::fs::OpenOptions::new().read(true).write(true).create_new(true).open(filename)?,
                    };
                    // Get the size of the file
                    let metadata = fd.metadata()?;

                    let size: JSValue = (metadata.len() as i32).try_into()?;
                    let result  = HashMap::from([("size", size)]);
                    // TODO add the others as needed
                    // result.insert("size", size);
                    Ok(JSValue::from_hashmap(result))
                } else {                    
                    let result  = HashMap::from([("size", -1)]);
                    // Just emit a message, be quiet
                    eprintln!("[WASM] warning {} has no read permissions", filename);
                    Ok(JSValue::from_hashmap(result))
                }
            })?,
        )?;

        context.eval_global("fs.js", include_str!("fs.js"))?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use crate::{fs::FS, APIConfig, JSApiSet};
    use anyhow::Result;
    use javy::Runtime;

    #[test]
    fn test_fs_write() -> Result<()> {
        let runtime = Runtime::default();
        let mut config = &mut APIConfig::default();
        // Glob pattern for any test file
        config.fs.add_to_write_whitelist("test.*");
        FS.register(&runtime, &config)?;
        let ctx = runtime.context();
        ctx.eval_global("test.js", "result = Node.FS.writeFileSync('test.txt', new Uint8Array([83]), 'w')")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert_eq!(result, 1);
        Ok(())
    }


    #[test]
    fn test_fs_write_forbidden() -> Result<()> {
        let runtime = Runtime::default();
        let mut config = &mut APIConfig::default();
        // Glob pattern for any test file
        config.fs.add_to_write_whitelist("testa.*");
        FS.register(&runtime, &config)?;
        let ctx = runtime.context();
        ctx.eval_global("test.js", "result = Node.FS.writeFileSync('test.txt', new Uint8Array([83]))")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert_eq!(result, 0);
        Ok(())
    }


    #[test]
    fn test_fs_write_ok_to_root() -> Result<()> {
        let runtime = Runtime::default();
        let mut config = &mut APIConfig::default();
        // Glob pattern for any test file
        config.fs.add_to_write_whitelist("./*");
        FS.register(&runtime, &config)?;
        let ctx = runtime.context();
        ctx.eval_global("test.js", "result = Node.FS.writeFileSync('./test.txt', new Uint8Array([83]), 'w')")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert_eq!(result, 1);
        Ok(())
    }


    #[test]
    fn test_fs_write_ok_to_root_but_dot_test_is_not_allowed() -> Result<()> {
        let runtime = Runtime::default();
        let mut config = &mut APIConfig::default();
        // Glob pattern for any test file
        config.fs.add_to_write_whitelist("./*");
        config.fs.add_to_write_blacklist("./.test");

        FS.register(&runtime, &config)?;
        let ctx = runtime.context();
        ctx.eval_global("test.js", "result = Node.FS.writeFileSync('./.test', new Uint8Array([83]), 'w')")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert_eq!(result, 0);

        ctx.eval_global("test.js", "result = Node.FS.writeFileSync('./test.txt', new Uint8Array([83]), 'w')")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert_eq!(result, 1);
        Ok(())
    }


    #[test]
    fn test_fs_write_with_options() -> Result<()> {
        let runtime = Runtime::default();
        let mut config = &mut APIConfig::default();
        // Glob pattern for any test file
        config.fs.add_to_write_whitelist("./*");
        config.fs.add_to_read_whitelist("./*");

        FS.register(&runtime, &config)?;
        let ctx = runtime.context();
        ctx.eval_global("test.js", "result = Node.FS.writeFileSync('./test.txt', new Uint8Array([83]),  'a+')")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert_eq!(result, 1);
        eprintln!("Written");
        // Read the content of the file, should be 2 bytes
        ctx.eval_global("test.js", "result = Node.FS.readFileSync('./test.txt', 'r')")?;
        let result = ctx.global_object()?.get_property("result")?.try_as_integer()?;
        assert!(result > 2);
        Ok(())
    }
}
