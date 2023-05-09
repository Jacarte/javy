use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct NodeRed;

extern "Rust" {

    /// Returns the size of the node red message encoded as JSON
    pub fn node_red_msg_size() -> usize;

    /// Ask the host to write the message JSON in the buff
    pub fn node_red_msg(buff: Vec<u8>, offset: i32, length: i32) -> usize; // Implemented by the host

    /// Send a custom message using the node red host
    pub fn node_red_send(data: Vec<u8>, offset: i32, length: i32);

    /// Call the done function of the node red host
    pub fn node_red_done(data: Vec<u8>, offset: i32, length: i32);
}

impl JSApiSet for NodeRed {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        let mut javy_object = global.get_property("Node")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("Node", javy_object)?;
        }

        global.set_property(
            "__node_msg",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };

                // The host writes the data in the memory

                // the data is a json file...we parse here

                // Parse the data
                let length = unsafe { node_red_msg(data.try_into()?, offset.try_into()?, length.try_into()?) };
                Ok(data.to_string().into())
            })?,
        )?;

        global.set_property("__node_msg_length", context.wrap_callback(|_, _this_arg, args|{
            let length = unsafe{   node_red_msg_size() };
            Ok(length.into())
            })?,
        )?;

        global.set_property(
            "__node_send",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length ,..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                unsafe { node_red_send(data.try_into()?, offset.try_into()?, length.try_into()?) };

                Ok(1.into())
            })?,
        )?;

        global.set_property(
            "__node_done",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length,  ..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                unsafe { node_red_done(data.try_into()?, offset.try_into()?, length.try_into()?) };

                Ok(1.into())
            })?,
        )?;

        context.eval_global("node_red.js", include_str!("node_red.js"))?;
        Ok(())
    }
}
