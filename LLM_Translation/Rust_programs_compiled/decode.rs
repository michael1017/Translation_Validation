use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use std::str;
use std::cmp::Ordering;

static URL_SCHEMES: [&'static str; 4] = ["aaa", "javascript", "jdbc", "doi"];

fn strff(ptr: *const c_char, n: isize) -> CString {
    let mut offset_ptr = ptr;
    for _ in 0..n {
        unsafe {
            offset_ptr = offset_ptr.offset(1);
        }
    }
    let c_str = unsafe { CString::from_raw(offset_ptr as *mut c_char) };
    c_str
}

fn strrwd(ptr: *const c_char, n: isize) -> CString {
    let mut offset_ptr = ptr;
    for _ in 0..n {
        unsafe {
            offset_ptr = offset_ptr.offset(-1);
        }
    }
    let c_str = unsafe { CString::from_raw(offset_ptr as *mut c_char) };
    c_str
}

fn url_is_protocol(str: &str) -> bool {
    URL_SCHEMES.iter().any(|&protocol| protocol == str)
}

fn url_is_ssh(str: &str) -> bool {
    str == "ssh" || str == "git"
}

fn get_part(url: &str, format: &str, l: usize) -> Option<String> {
    let tmp_url = &url[l..];
    if let Some(idx) = tmp_url.find(format) {
        Some(tmp_url[..idx].to_string())
    } else {
        None
    }
}

fn url_get_protocol(url: &str) -> Option<String> {
    let protocol_end = url.find("://")?;
    let protocol = &url[..protocol_end];
    if url_is_protocol(protocol) {
        Some(protocol.to_string())
    } else {
        None
    }
}

fn url_get_auth(url: &str) -> Option<String> {
    let protocol = url_get_protocol(url)?;
    let l = protocol.len() + 3;
    get_part(url, "@", l)
}

fn url_get_hostname(url: &str) -> Option<String> {
    let protocol = url_get_protocol(url)?;
    let auth = url_get_auth(url);
    let l = protocol.len() + 3 + auth.as_ref().map_or(0, |a| a.len() + 1);
    let format = if url_is_ssh(&protocol) { ":" } else { "/" };
    get_part(url, format, l)
}

fn url_get_path(url: &str) -> Option<String> {
    let protocol = url_get_protocol(url)?;
    let auth = url_get_auth(url);
    let hostname = url_get_hostname(url)?;
    let l = protocol.len() + 3 + hostname.len() + auth.as_ref().map_or(0, |a| a.len() + 1);
    let is_ssh = url_is_ssh(&protocol);
    let part = if is_ssh { "%s" } else { "/%s" };
    let mut tmp_path = get_part(url, part, l)?;
    if !is_ssh {
        tmp_path.insert(0, '/');
    }
    Some(tmp_path)
}

fn main() {
    let url = "http://user:pass@subdomain.host.com:8080/p/a/t/h?query=string#hash";
    assert_eq!(Some("/p/a/t/h?query=string#hash".to_string()), url_get_path(url));
}