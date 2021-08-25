#![allow(clippy::wrong_self_convention)]
use quickjs_sys::*;
use std::{ffi::CString, os::raw::c_char, os::raw::c_int, os::raw::c_void, ptr};

#[derive(Debug, Copy, Clone)]
pub struct Context {
    pub raw: *mut JSContext,
    pub rt: *mut JSRuntime,
}

// TODO
// Extract the 'pure value' functions

impl Default for Context {
    fn default() -> Self {
        let rt = unsafe { JS_NewRuntime() };
        if rt.is_null() {
            panic!("failed to initialize js runtime");
        }

        let context = unsafe { JS_NewContext(rt) };
        if context.is_null() {
            // Free the runtime
            panic!("failed to initialize js context");
        }

        Self { raw: context, rt }
    }
}

impl Context {
    pub fn eval(&self, bytes: &[u8], name: &str) -> JSValue {
        let input = make_cstring(bytes.to_vec());
        let script_name = make_cstring(name);

        unsafe {
            JS_Eval(
                self.raw,
                input.as_ptr(),
                (bytes.len() - 1) as _,
                script_name.as_ptr(),
                JS_EVAL_TYPE_GLOBAL as i32,
            )
        }
    }

    pub fn global(&self) -> JSValue {
        unsafe { JS_GetGlobalObject(self.raw) }
    }

    pub fn call(&self, fun: JSValue, from: JSValue, args: &[JSValue]) -> JSValue {
        unsafe {
            JS_Call(
                self.raw,
                fun,
                from,
                args.len() as i32,
                args.as_ptr() as *mut u64,
            )
        }
    }

    pub unsafe fn new_float64(&self, val: f64) -> JSValue {
        JS_NewFloat64_Ext(self.raw, val)
    }

    pub unsafe fn is_float64(&self, val: JSValue) -> bool {
        JS_IsFloat64_Ext(self.get_tag(val)) == 1
    }

    pub unsafe fn to_float64(&self, val: JSValue) -> f64 {
        let mut ret = 0 as f64;
        JS_ToFloat64(self.raw, &mut ret, val);
        ret
    }

    pub unsafe fn new_bool(&self, val: bool) -> JSValue {
        JS_NewBool_Ext(self.raw, val as i32)
    }

    pub fn new_string(&self, val: &str) -> JSValue {
        unsafe { JS_NewStringLen(self.raw, val.as_ptr() as *const c_char, val.len() as _) }
    }

    pub fn new_array(&self) -> JSValue {
        unsafe { JS_NewArray(self.raw) }
    }

    pub unsafe fn new_int32(&self, val: i32) -> JSValue {
        JS_NewInt32_Ext(self.raw, val)
    }

    pub unsafe fn new_uint32(&self, val: u32) -> JSValue {
        JS_NewUint32_Ext(self.raw, val)
    }

    pub fn new_object(&self) -> JSValue {
        unsafe { JS_NewObject(self.raw) }
    }

    pub fn get_str_property(&self, name: &str, from: JSValue) -> JSValue {
        unsafe { JS_GetPropertyStr(self.raw, from, make_cstring(name).as_ptr()) }
    }

    pub fn set_str_property(&self, target: JSValue, key: &str, val: JSValue) {
        let key_name = make_cstring(key);
        unsafe {
            JS_DefinePropertyValueStr(
                self.raw,
                target,
                key_name.as_ptr(),
                val,
                JS_PROP_C_W_E as i32,
            );
        }
    }

    pub fn set_property_raw(&self, target: JSValue, key: *const i8, val: JSValue) {
        unsafe { JS_DefinePropertyValueStr(self.raw, target, key, val, JS_PROP_C_W_E as i32) };
    }

    pub fn set_uint32_property(&self, target: JSValue, val: JSValue) {
        unsafe {
            let len = self.get_str_property("length", target);
            JS_DefinePropertyValueUint32(self.raw, target, len as u32, val, JS_PROP_C_W_E as i32);
        }
    }

    pub fn get_uint32_property(&self, target: JSValue, at: u32) -> JSValue {
        unsafe { JS_GetPropertyUint32(self.raw, target, at) }
    }

    pub fn get_own_properties(&self, obj: JSValue) -> (*mut JSPropertyEnum, i32) {
        let flags = (JS_GPN_STRING_MASK | JS_GPN_SYMBOL_MASK | JS_GPN_ENUM_ONLY) as i32;
        let mut properties: *mut JSPropertyEnum = ptr::null_mut();
        let mut count = 0;

        let result =
            unsafe { JS_GetOwnPropertyNames(self.raw, &mut properties, &mut count, obj, flags) };
        assert!(result == 0);

        (properties, count as i32)
    }

    pub fn get_internal_property(&self, obj: JSValue, key: JSAtom) -> JSValue {
        unsafe { JS_GetPropertyInternal(self.raw, obj, key, obj, 0) }
    }

    pub fn json_parse(&self, json_string: &str) -> JSValue {
        let buf = json_string.as_ptr() as *const std::os::raw::c_char;
        unsafe {
            JS_ParseJSON(
                self.raw,
                buf,
                json_string.len() as _,
                make_cstring("input").as_ptr(),
            )
        }
    }

    pub fn json_stringify(&self, obj: JSValue) -> JSValue {
        unsafe {
            JS_JSONStringify(
                self.raw,
                obj,
                JS_TAG_UNDEFINED as u64,
                JS_TAG_UNDEFINED as u64,
            )
        }
    }

    pub fn get_tag(&self, val: JSValue) -> i32 {
        (val >> 32) as i32
    }

    pub fn to_string(&self, val: JSValue) -> String {
        let string = unsafe { JS_ToString(self.raw, val) };
        self.deserialize_string(string)
    }

    pub fn atom_to_string(&self, atom: JSAtom) -> JSValue {
        unsafe { JS_AtomToString(self.raw, atom) }
    }

    pub fn to_byte_slice(&self, val: JSValue) -> &[u8] {
        unsafe {
            let mut len: size_t = 0;
            let ptr = JS_ToCStringLen2(self.raw, &mut len, val, 0);
            let ptr = ptr as *const u8;
            let len = len as usize;
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn deserialize_string(&self, val: JSValue) -> String {
        let cstr = self.to_byte_slice(val);
        std::str::from_utf8(cstr).unwrap().to_string()
    }

    pub fn is_exception(&self, val: JSValue) -> bool {
        self.get_tag(val) == JS_TAG_EXCEPTION
    }

    pub fn is_array(&self, val: JSValue) -> bool {
        unsafe { JS_IsArray(self.raw, val) > 0 }
    }

    pub fn is_null(&self, val: JSValue) -> bool {
        self.get_tag(val) == JS_TAG_NULL
    }

    pub fn is_bool(&self, val: JSValue) -> bool {
        self.get_tag(val) == JS_TAG_BOOL
    }

    pub fn is_integer(&self, val: JSValue) -> bool {
        self.get_tag(val) == JS_TAG_INT
    }

    pub fn is_string(&self, val: JSValue) -> bool {
        self.get_tag(val) == JS_TAG_STRING
    }

    /// Creates a new JSValue that can be registered as a callback.
    ///
    /// This operation is unsafe since it's up to the caller to ensure that the environment used by the closure
    /// must live as long as the JSRuntime.
    pub unsafe fn new_callback<F>(&self, f: F) -> JSValue
    where
        F: FnMut(*mut JSContext, JSValue, c_int, *mut JSValue, c_int) -> JSValue,
    {
        // Lifetime is not respected and behavior is undefined. If we truly want to support
        // closure and capture the environment, it must live as long as &self.
        //
        // The following example will not produce the expected result:
        //
        // ```rs
        // let bar = "bar".to_string();
        // self.create_callback(|_, _, _, _, _| println!("foo: {}", &bar));
        // ```
        let trampoline = build_trampoline(&f);
        let data = &f as *const _ as *mut c_void as *mut JSValue;

        JS_NewCFunctionData(self.raw, trampoline, 0, 1, 1, data)
    }

    pub fn set_property(&self, receiver: JSValue, name: impl Into<Vec<u8>>, value: JSValue) {
        unsafe {
            JS_SetPropertyStr(self.raw, receiver, make_cstring(name).as_ptr(), value);
        }
    }
}

fn build_trampoline<F>(_f: &F) -> JSCFunctionData
where
    F: FnMut(*mut JSContext, JSValue, c_int, *mut JSValue, c_int) -> JSValue,
{
    // We build a trampoline to jump between c <-> rust and allow closing over a specific context.
    // For more info around how this works, see https://adventures.michaelfbryan.com/posts/rust-closures-in-ffi/.
    unsafe extern "C" fn trampoline<F>(
        ctx: *mut JSContext,
        this: JSValue,
        argc: c_int,
        argv: *mut JSValue,
        magic: c_int,
        data: *mut JSValue,
    ) -> JSValue
    where
        F: FnMut(*mut JSContext, JSValue, c_int, *mut JSValue, c_int) -> JSValue,
    {
        let closure_ptr = data;
        let closure: &mut F = &mut *(closure_ptr as *mut F);
        (*closure)(ctx, this, argc, argv, magic)
    }

    Some(trampoline::<F>)
}

fn make_cstring(value: impl Into<Vec<u8>>) -> CString {
    CString::new(value).unwrap()
}
