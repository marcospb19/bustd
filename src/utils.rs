use std::fs::File;
use std::{ffi::CStr, mem, ptr};

use libc::sysconf;
use libc::_SC_PAGESIZE;
use libc::{geteuid, getpwuid_r, passwd};

use crate::error::{Error, Result};

pub fn page_size() -> Result<i64> {
    let page_size = unsafe { sysconf(_SC_PAGESIZE) };
    if page_size == -1 {
        return Err(Error::SysconfFailedError);
    }

    // The type of page_size differs between architectures
    // so we use .into() to convert to i64 if necessary
    Ok(page_size.into())
}

pub unsafe fn get_username() -> Option<String> {
    let mut buf = [0; 2048];
    let mut result = ptr::null_mut();
    let mut passwd: passwd = mem::zeroed();

    // Gets the effective user ID of the calling process
    // The POSIX Programmer's Manual states this function shouldn't fail here
    let uid = geteuid();

    let getpwuid_r_code = getpwuid_r(uid, &mut passwd, buf.as_mut_ptr(), buf.len(), &mut result);

    if getpwuid_r_code == 0 && !result.is_null() {
        // If getpwuid_r succeeded, let's get the username from it
        let username = CStr::from_ptr(passwd.pw_name);
        let username = String::from_utf8_lossy(username.to_bytes());

        return Some(username.into());
    }

    None
}

pub fn str_from_u8(buf: &[u8]) -> Result<&str> {
    let first_nul_idx = buf.iter().position(|&c| c == b'\0').unwrap_or(buf.len());

    Ok(std::str::from_utf8(&buf[0..first_nul_idx])?)
}

// pub fn file_from_buffer(buf: [u8; 50]) -> Result<([u8; 50], File)> {
//     let path = str_from_u8(&buf)?;
//     let mut file = File::open(&path)?;
//     Ok((buf, file))
// }

pub fn file_from_buffer(buf: &[u8]) -> Result<File> {
    let path = str_from_u8(&buf)?;
    let file = File::open(&path)?;
    Ok(file)
}
