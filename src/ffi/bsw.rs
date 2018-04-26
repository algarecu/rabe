use schemes::bsw::*;
use std::ops::Deref;
use libc::*;
use std::ffi::CStr;
use std::mem::transmute;
use std::string::String;
use serde_json;

#[no_mangle]
pub extern "C" fn bsw_context_create() -> *mut CpAbeContext {
    let _ctx = unsafe { transmute(Box::new(setup())) };
    _ctx
}

#[no_mangle]
pub extern "C" fn bsw_context_destroy(ctx: *mut CpAbeContext) {
    let _ctx: Box<CpAbeContext> = unsafe { transmute(ctx) };
    _ctx.deref();
}

#[no_mangle]
pub extern "C" fn bsw_keygen(
    ctx: *mut CpAbeContext,
    attributes: *mut Vec<String>,
) -> *mut CpAbeSecretKey {
    let _attr = unsafe { &mut *attributes };
    let attr_vec: Vec<_> = _attr.iter().map(|arg| arg.to_string()).collect();
    let _ctx = unsafe { &*ctx };
    let _sk = unsafe {
        transmute(Box::new(
            keygen(&_ctx._pk, &_ctx._msk, &attr_vec).unwrap().clone(),
        ))
    };
    _sk
}

#[no_mangle]
pub extern "C" fn bsw_keygen_destroy(sk: *mut CpAbeSecretKey) {
    let _sk: Box<CpAbeSecretKey> = unsafe { transmute(sk) };
    _sk.deref();
}

#[no_mangle]
pub extern "C" fn bsw_delegate(
    ctx: *mut CpAbeContext,
    sk: *mut CpAbeSecretKey,
    attributes: *mut Vec<String>,
) -> *mut CpAbeSecretKey {
    let _attr = unsafe { &mut *attributes };
    let attr_vec: Vec<_> = _attr.iter().map(|arg| arg.to_string()).collect();
    let _ctx = unsafe { &*ctx };
    let _sk = unsafe { &*sk };
    let _dsk = unsafe { transmute(Box::new(delegate(&_ctx._pk, &_sk, &attr_vec).unwrap())) };
    _dsk
}

#[no_mangle]
pub extern "C" fn bsw_encrypt_size(
    ctx: *mut CpAbeContext,
    policy: *mut c_char,
    data: *mut u8,
    data_len: u32,
) -> i32 {
    let p = unsafe { &mut *policy };
    let mut _pol = unsafe { CStr::from_ptr(p) };
    let pol = String::from(_pol.to_str().unwrap());
    unsafe {
        let _ctx = &*ctx;
        let _data = &mut *data;
        let _data_vec = Vec::from_raw_parts(data, data_len as usize, data_len as usize);
        let _ct = encrypt(&_ctx._pk, &pol, &_data_vec).unwrap();
        let _ct_str = serde_json::to_string_pretty(&_ct).unwrap();
        _ct_str.len() as i32
    }
}

#[no_mangle]
pub extern "C" fn bsw_encrypt(
    ctx: *mut CpAbeContext,
    policy: *mut c_char,
    data: *mut u8,
    data_len: u32,
    buf: *mut u8,
    buf_len: *mut u32,
) {
    let p = unsafe { &mut *policy };
    let mut _pol = unsafe { CStr::from_ptr(p) };
    let pol = String::from(_pol.to_str().unwrap());
    let _ctx = unsafe { &*ctx };
    let _data = unsafe { &mut *data };
    unsafe {
        let _ctx = &*ctx;
        let _data = &mut *data;
        let _data_vec = Vec::from_raw_parts(data, data_len as usize, data_len as usize);
        let _ct = encrypt(&_ctx._pk, &pol, &_data_vec).unwrap();
        let _ct_str = serde_json::to_string_pretty(&_ct).unwrap();
    }
    use std::{slice, ptr};
    unsafe {
        ptr::copy_nonoverlapping(data, buf, data_len as usize);
    }
}

#[no_mangle]
pub extern "C" fn bsw_decrypt(sk: *mut CpAbeSecretKey, ct: *mut CpAbeCiphertext) -> Vec<u8> {
    let _sk = unsafe { &mut *sk };
    let _ct = unsafe { &mut *ct };
    let pt = decrypt(&_sk, &_ct).unwrap();
    pt
}