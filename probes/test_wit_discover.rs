#!/usr/bin/env rustc
//! Runtime WIT type discovery
//!
//! Loads a library, reads embedded WIT metadata, and discovers types at runtime.
//!
//! Compile and run:
//!   rustc --edition 2021 --crate-type cdylib test_wit_lib.rs -o libwit_test.dylib
//!   rustc --edition 2021 test_wit_discover.rs -o test_wit_discover
//!   DYLD_LIBRARY_PATH=. ./test_wit_discover

use std::ffi::{c_char, c_int, c_void, CStr, CString};
use std::collections::HashMap;

// =============================================================================
// Simple WIT Parser (subset for prototype)
// =============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum WitType {
    S8, S16, S32, S64,
    U8, U16, U32, U64,
    F32, F64,
    Bool,
    String,
    List(Box<WitType>),
    Option(Box<WitType>),
    Record(String),
    Unknown(String),
}

#[derive(Debug, Clone)]
pub struct WitParam {
    pub name: String,
    pub ty: WitType,
}

#[derive(Debug, Clone)]
pub struct WitFunc {
    pub name: String,
    pub params: Vec<WitParam>,
    pub result: Option<WitType>,
}

#[derive(Debug, Clone)]
pub struct WitField {
    pub name: String,
    pub ty: WitType,
}

#[derive(Debug, Clone)]
pub struct WitRecord {
    pub name: String,
    pub fields: Vec<WitField>,
}

#[derive(Debug, Clone)]
pub struct WitInterface {
    pub name: String,
    pub functions: Vec<WitFunc>,
    pub records: Vec<WitRecord>,
}

#[derive(Debug, Clone)]
pub struct WitPackage {
    pub name: String,
    pub version: Option<String>,
    pub interfaces: Vec<WitInterface>,
}

fn parse_type(s: &str) -> WitType {
    let s = s.trim();
    match s {
        "s8" => WitType::S8,
        "s16" => WitType::S16,
        "s32" => WitType::S32,
        "s64" => WitType::S64,
        "u8" => WitType::U8,
        "u16" => WitType::U16,
        "u32" => WitType::U32,
        "u64" => WitType::U64,
        "f32" => WitType::F32,
        "f64" => WitType::F64,
        "bool" => WitType::Bool,
        "string" => WitType::String,
        _ if s.starts_with("list<") && s.ends_with(">") => {
            let inner = &s[5..s.len()-1];
            WitType::List(Box::new(parse_type(inner)))
        }
        _ if s.starts_with("option<") && s.ends_with(">") => {
            let inner = &s[7..s.len()-1];
            WitType::Option(Box::new(parse_type(inner)))
        }
        _ => WitType::Unknown(s.to_string()),
    }
}

fn parse_func(line: &str) -> Option<WitFunc> {
    // Parse: "name: func(param: type, ...) -> result"
    let line = line.trim().trim_end_matches(';');
    if !line.contains(": func(") {
        return None;
    }

    let parts: Vec<&str> = line.splitn(2, ": func(").collect();
    if parts.len() != 2 {
        return None;
    }

    let name = parts[0].trim().replace("-", "_"); // WIT uses kebab-case

    let rest = parts[1];
    let (params_str, result_str) = if let Some(idx) = rest.find(") ->") {
        (&rest[..idx], Some(rest[idx + 4..].trim()))
    } else if rest.ends_with(")") {
        (&rest[..rest.len()-1], None)
    } else {
        return None;
    };

    let mut params = Vec::new();
    if !params_str.is_empty() {
        for param in params_str.split(',') {
            let param = param.trim();
            if let Some(idx) = param.find(':') {
                let pname = param[..idx].trim().replace("-", "_");
                let ptype = parse_type(&param[idx+1..]);
                params.push(WitParam { name: pname, ty: ptype });
            }
        }
    }

    let result = result_str.map(|s| parse_type(s));

    Some(WitFunc { name, params, result })
}

fn parse_record(lines: &[&str], start: usize) -> Option<(WitRecord, usize)> {
    let first = lines[start].trim();
    if !first.starts_with("record ") {
        return None;
    }

    let name = first[7..].trim().trim_end_matches('{').trim().to_string();
    let mut fields = Vec::new();
    let mut i = start + 1;

    while i < lines.len() {
        let line = lines[i].trim();
        if line == "}" {
            break;
        }
        if line.contains(':') && !line.contains("func") {
            let parts: Vec<&str> = line.trim_end_matches(',').splitn(2, ':').collect();
            if parts.len() == 2 {
                let fname = parts[0].trim().replace("-", "_");
                let ftype = parse_type(parts[1]);
                fields.push(WitField { name: fname, ty: ftype });
            }
        }
        i += 1;
    }

    Some((WitRecord { name, fields }, i))
}

pub fn parse_wit(source: &str) -> WitPackage {
    let lines: Vec<&str> = source.lines().collect();
    let mut package_name = String::new();
    let mut version = None;
    let mut interfaces = Vec::new();
    let mut current_interface: Option<WitInterface> = None;

    let mut i = 0;
    while i < lines.len() {
        let line = lines[i].trim();

        // Skip empty lines and comments
        if line.is_empty() || line.starts_with("//") {
            i += 1;
            continue;
        }

        // Package declaration
        if line.starts_with("package ") {
            let pkg = line[8..].trim_end_matches(';').trim();
            if let Some(at_idx) = pkg.find('@') {
                package_name = pkg[..at_idx].to_string();
                version = Some(pkg[at_idx+1..].to_string());
            } else {
                package_name = pkg.to_string();
            }
        }
        // Interface start
        else if line.starts_with("interface ") {
            if let Some(iface) = current_interface.take() {
                interfaces.push(iface);
            }
            let iname = line[10..].trim().trim_end_matches('{').trim();
            current_interface = Some(WitInterface {
                name: iname.to_string(),
                functions: Vec::new(),
                records: Vec::new(),
            });
        }
        // Interface end
        else if line == "}" {
            if let Some(iface) = current_interface.take() {
                interfaces.push(iface);
            }
        }
        // Record
        else if line.starts_with("record ") {
            if let Some((record, end)) = parse_record(&lines, i) {
                if let Some(ref mut iface) = current_interface {
                    iface.records.push(record);
                }
                i = end;
            }
        }
        // Function
        else if line.contains(": func(") {
            if let Some(func) = parse_func(line) {
                if let Some(ref mut iface) = current_interface {
                    iface.functions.push(func);
                }
            }
        }

        i += 1;
    }

    if let Some(iface) = current_interface {
        interfaces.push(iface);
    }

    WitPackage {
        name: package_name,
        version,
        interfaces,
    }
}

// =============================================================================
// Dynamic Library Loading
// =============================================================================

const RTLD_NOW: c_int = 0x2;

extern "C" {
    fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> c_int;
    fn dlerror() -> *mut c_char;
}

struct DynLib(*mut c_void);

impl DynLib {
    fn open(path: &str) -> Result<Self, String> {
        let cpath = CString::new(path).unwrap();
        let handle = unsafe { dlopen(cpath.as_ptr(), RTLD_NOW) };
        if handle.is_null() {
            let err = unsafe { dlerror() };
            let msg = if err.is_null() {
                "unknown error".to_string()
            } else {
                unsafe { CStr::from_ptr(err).to_string_lossy().to_string() }
            };
            return Err(msg);
        }
        Ok(Self(handle))
    }

    fn sym(&self, name: &str) -> *mut c_void {
        let cname = CString::new(name).unwrap();
        unsafe { dlsym(self.0, cname.as_ptr()) }
    }

    fn get_wit_types(&self) -> Option<&'static str> {
        let ptr = self.sym("WIT_TYPES") as *const *const u8;
        if ptr.is_null() {
            return None;
        }
        let len_ptr = self.sym("WIT_TYPES_LEN") as *const usize;
        if len_ptr.is_null() {
            return None;
        }
        unsafe {
            let data = *ptr;
            let len = *len_ptr;
            let slice = std::slice::from_raw_parts(data, len);
            std::str::from_utf8(slice).ok()
        }
    }
}

impl Drop for DynLib {
    fn drop(&mut self) {
        unsafe { dlclose(self.0); }
    }
}

// =============================================================================
// Main: Runtime Type Discovery
// =============================================================================

fn main() {
    println!("=== WIT Runtime Type Discovery ===\n");

    // Load the library
    let lib = DynLib::open("./libwit_test.dylib").expect("Failed to load library");
    println!("Loaded: libwit_test.dylib\n");

    // Read embedded WIT
    let wit_source = lib.get_wit_types().expect("No WIT_TYPES symbol found");
    println!("--- Embedded WIT ---");
    println!("{}", wit_source);

    // Parse WIT at runtime
    let package = parse_wit(wit_source);
    println!("--- Parsed Package ---");
    println!("Package: {}", package.name);
    if let Some(ver) = &package.version {
        println!("Version: {}", ver);
    }
    println!();

    // Discover interfaces, records, and functions
    for iface in &package.interfaces {
        println!("Interface: {}", iface.name);

        for record in &iface.records {
            println!("  Record: {}", record.name);
            for field in &record.fields {
                println!("    {}: {:?}", field.name, field.ty);
            }
        }

        for func in &iface.functions {
            let params: Vec<String> = func.params.iter()
                .map(|p| format!("{}: {:?}", p.name, p.ty))
                .collect();
            let result = func.result.as_ref()
                .map(|r| format!(" -> {:?}", r))
                .unwrap_or_default();
            println!("  func {}({}){}", func.name, params.join(", "), result);
        }
        println!();
    }

    // Actually call the discovered functions
    println!("--- Calling Discovered Functions ---");

    // Find and call 'add'
    if package.interfaces.iter().any(|i| i.functions.iter().any(|f| f.name == "add")) {
        let add: extern "C" fn(i32, i32) -> i32 = unsafe { std::mem::transmute(lib.sym("add")) };
        println!("add(10, 20) = {}", add(10, 20));
    }

    // Find and call 'multiply'
    if package.interfaces.iter().any(|i| i.functions.iter().any(|f| f.name == "multiply")) {
        let multiply: extern "C" fn(f64, f64) -> f64 = unsafe { std::mem::transmute(lib.sym("multiply")) };
        println!("multiply(3.5, 2.0) = {}", multiply(3.5, 2.0));
    }

    // Find and call 'factorial'
    if package.interfaces.iter().any(|i| i.functions.iter().any(|f| f.name == "factorial")) {
        let factorial: extern "C" fn(u32) -> u64 = unsafe { std::mem::transmute(lib.sym("factorial")) };
        println!("factorial(10) = {}", factorial(10));
    }

    // Find and call 'greet'
    if package.interfaces.iter().any(|i| i.functions.iter().any(|f| f.name == "greet")) {
        let greet: extern "C" fn(*const c_char) -> *mut c_char = unsafe { std::mem::transmute(lib.sym("greet")) };
        let free_string: extern "C" fn(*mut c_char) = unsafe { std::mem::transmute(lib.sym("free_string")) };

        let name = CString::new("Runtime Discovery").unwrap();
        let result = greet(name.as_ptr());
        let greeting = unsafe { CStr::from_ptr(result).to_str().unwrap() };
        println!("greet(\"Runtime Discovery\") = {}", greeting);
        free_string(result);
    }

    // Find and call 'sum_array'
    if package.interfaces.iter().any(|i| i.functions.iter().any(|f| f.name == "sum_array")) {
        let sum_array: extern "C" fn(*const i32, usize) -> i64 = unsafe { std::mem::transmute(lib.sym("sum_array")) };
        let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        println!("sum_array([1..10]) = {}", sum_array(nums.as_ptr(), nums.len()));
    }

    println!("\n=== All tests passed! ===");
}
