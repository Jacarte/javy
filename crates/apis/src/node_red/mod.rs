use anyhow::Result;
use std::io::{Read, Write};

use javy::Runtime;

use crate::{APIConfig, JSApiSet};

pub(super) struct NodeRed;

extern "Rust" {

    /// Returns the size of the node red message encoded as JSON
    #[allow(dead_code)]
    pub fn node_red_msg_size() -> usize;

    /// Ask the host to write the message JSON in the buff
    #[allow(dead_code)]
    pub fn node_red_msg(data: *const u8, offset: i32, length: i32) -> usize; // Implemented by the host

    /// Send a custom message using the node red host
    #[allow(dead_code)]
    pub fn node_red_send(data: *const u8, offset: i32, length: i32);

    /// Send a custom message using the node red host
    #[allow(dead_code)]
    pub fn node_red_warn(data: *const u8, offset: i32, length: i32);

    /// Error a custom message using the node red host
    #[allow(dead_code)]
    pub fn node_red_error(data: *const u8, offset: i32, length: i32);

    /// Call the done function of the node red host
    #[allow(dead_code)]
    pub fn node_red_done(data: *const u8, offset: i32, length: i32);


    /// Returns the size of the node red node struct encoded as JSON
    #[allow(dead_code)]
    pub fn node_red_node_length() -> usize;

    /// Ask the host to write the node JSON in the buff
    #[allow(dead_code)]
    pub fn node_red_node(data: *const u8, offset: i32, length: i32) -> usize; // Implemented by the host
    
    #[allow(dead_code)]
    pub fn node_red_result(data: *const u8, offset: i32, length:i32);

    #[allow(dead_code)]
    pub fn node_red_register(data: *const u8, offset: i32, length:i32);
    
    #[allow(dead_code)]
    pub fn node_emit(data: *const u8, offset: i32, length: i32);

    /// Returns the size of the node red message encoded as JSON
    #[allow(dead_code)]
    pub fn node_red_pop_size() -> usize;

    /// Ask the host to write the message JSON in the buff
    #[allow(dead_code)]
    pub fn node_red_pop(data: *const u8, offset: i32, length: i32) -> usize; // Implemented by the host


    /// Returns the size of the node context encoded as JSON
    #[allow(dead_code)]
    pub fn node_red_context_size() -> usize;

    /// Ask the host to write the context in the buff
    #[allow(dead_code)]
    pub fn node_red_context(data: *const u8, offset: i32, length: i32) -> usize; // Implemented by the host

}

impl JSApiSet for NodeRed {
    fn register(&self, runtime: &Runtime, _config: &APIConfig) -> Result<()> {
        let context = runtime.context();
        let global = context.global_object()?;

        // Define some patched globals

        let mut javy_object = global.get_property("process")?;
        if javy_object.is_undefined() {
            javy_object = context.object_value()?;
            global.set_property("process", javy_object)?;
        }

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


                // Parse the data
                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                let data = data.as_bytes_mut()?;
                // The host writes the data in the memory
                let length = unsafe { node_red_msg(data.as_ptr().try_into()?, offset.try_into()?, length.try_into()?) };
                // TODO FIX this
                Ok(String::from(std::str::from_utf8(&*data).unwrap()).into())
            })?,
        )?;

        global.set_property("__node_msg_length", context.wrap_callback(|_, _this_arg, args|{
                let length = unsafe{  node_red_msg_size() };
                Ok(length.into())
            })?,
        )?;

        global.set_property(
            "__node_pop",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };


                // Parse the data
                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                let data = data.as_bytes_mut()?;
                // The host writes the data in the memory
                let length = unsafe { node_red_pop(data.as_ptr().try_into()?, offset.try_into()?, length.try_into()?) };
                // TODO FIX this
                Ok(String::from(std::str::from_utf8(&*data).unwrap()).into())
            })?,
        )?;


        global.set_property(
            "__node_context",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };


                // Parse the data
                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                let data = data.as_bytes_mut()?;
                // The host writes the data in the memory
                let length = unsafe { node_red_context(data.as_ptr().try_into()?, offset.try_into()?, length.try_into()?) };
                // TODO FIX this
                Ok(String::from(std::str::from_utf8(&*data).unwrap()).into())
            })?,
        )?;

        global.set_property("__node_context_length", 
            context.wrap_callback(|_, _this_arg, args|{
                let length = unsafe{  node_red_context_size() };
                Ok(length.into())
            })?,
        )?;

        global.set_property("__node_pop_length", 
            context.wrap_callback(|_, _this_arg, args|{
                let length = unsafe{  node_red_pop_size() };
                Ok(length.into())
            })?,
        )?;

        global.set_property(
            "__node_node",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length, ..] = args else {
                    anyhow::bail!("Invalid number of parameters");
                };


                // Parse the data
                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                let data = data.as_bytes_mut()?;
                // The host writes the data in the memory
                let length = unsafe { node_red_node(data.as_ptr().try_into()?, offset.try_into()?, length.try_into()?) };
                // TODO FIX this
                Ok(String::from(std::str::from_utf8(&*data).unwrap()).into())
            })?,
        )?;

        global.set_property("__node_node_length", 
            context.wrap_callback(|_, _this_arg, args|{
                let length = unsafe{  node_red_node_length() };
                Ok(length.into())
            })?,
        )?;

        global.set_property(
            "__node_send",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length ,..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_red_send(data.try_into()?, offset.try_into()?, length.try_into()?) };

                Ok(1.into())
            })?,
        )?;


        global.set_property(
            "__node_emit",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length ,..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_emit(data.try_into()?, offset.try_into()?, length.try_into()?) };

                Ok(1.into())
            })?,
        )?;


        global.set_property(
            "__node_warn",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length ,..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_red_warn(data.try_into()?, offset.try_into()?, length.try_into()?) };

                Ok(1.into())
            })?,
        )?;



        global.set_property(
            "__node_error",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length ,..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_red_error(data.try_into()?, offset.try_into()?, length.try_into()?) };

                Ok(1.into())
            })?,
        )?;

        global.set_property(
            "__node_done",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length,  ..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }
                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_red_done(data.try_into()?, offset.try_into()?, length.try_into()?) };
                Ok(1.into())
            })?,
        )?;

        global.set_property(
            "__node_red_result",
            context.wrap_callback(|_, _this_arg, args| {
                let [data, offset, length,  ..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }

                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_red_result(data.try_into()?, offset.try_into()?, length.try_into()?) };
                Ok(1.into())
            })?,
        )?;

        global.set_property(
            "__node_red_register",
            context.wrap_callback(|_, _this_arg, args|{

                let [data, offset, length,  ..] = args else {
                    anyhow::bail!("Invalid number of parameters")
                };

                // let data = unsafe { data.inner_value() };
                if !data.is_array_buffer() {
                    anyhow::bail!("Data needs to be an ArrayBuffer");
                }

                // It does not need to be mut
                let data = data.as_bytes_mut()?.as_ptr();
                unsafe { node_red_register(data.try_into()?, offset.try_into()?, length.try_into()?) };
                Ok(1.into())
            })?,
        )?;

        context.eval_global("node_red.js", include_str!("node_red.js"))?;
        Ok(())
    }
}
