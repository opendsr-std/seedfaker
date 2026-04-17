#![allow(unsafe_code)]
#![deny(clippy::undocumented_unsafe_blocks)]

// C-ABI shared library for seedfaker.
// Loaded via FFI from PHP, Ruby, Go, C#, Java, and any language with C interop.
//
// Memory contract:
// - Every `*mut c_char` returned by `sf_*` functions must be freed with `sf_free()`.
// - `sf_create` returns an opaque handle; caller must call `sf_destroy` when done.
// - `sf_last_error` returns a thread-local pointer valid until the next `sf_*` call.

use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

use seedfaker_core::field;
use seedfaker_core::locale;
use seedfaker_core::pipeline;

pub struct SfFaker {
    seed: u64,
    locales: Vec<&'static locale::Locale>,
    record_counter: u64,
    tz_offset: i32,
    since: i64,
    until: i64,
}

thread_local! {
    static LAST_ERROR: RefCell<CString> = RefCell::new(CString::default());
}

fn set_error(msg: &str) {
    LAST_ERROR.with(|e| {
        // replace('\0', "") guarantees no interior nulls; CString::new cannot fail.
        let cleaned = msg.replace('\0', "");
        if let Ok(cs) = CString::new(cleaned) {
            *e.borrow_mut() = cs;
        }
    });
}

fn to_cstring(s: &str) -> *mut c_char {
    if let Ok(cs) = CString::new(s) {
        cs.into_raw()
    } else {
        set_error("output contains null byte");
        std::ptr::null_mut()
    }
}

/// Read a C string from a raw pointer, returning the default if NULL.
///
/// # Safety
/// `ptr` must be null or point to a valid NUL-terminated C string.
unsafe fn read_cstr(ptr: *const c_char, default: &str) -> Result<&str, ()> {
    if ptr.is_null() {
        return Ok(default);
    }
    // SAFETY: caller guarantees valid NUL-terminated string when non-null.
    unsafe { CStr::from_ptr(ptr) }.to_str().map_err(|_| ())
}

/// Create a Faker instance. `opts_json` is a JSON object:
/// `{"seed":"x","locale":"en,de","tz":"+0300","since":1990,"until":2025}`
/// All fields optional. Returns opaque handle or NULL on error (check `sf_last_error`).
///
/// # Safety
/// `opts_json` must be null or a valid NUL-terminated UTF-8 C string.
#[no_mangle]
pub unsafe extern "C" fn sf_create(opts_json: *const c_char) -> *mut SfFaker {
    // SAFETY: null or valid C string per function contract.
    let Ok(opts_str) = (unsafe { read_cstr(opts_json, "{}") }) else {
        set_error("opts_json is not valid UTF-8");
        return std::ptr::null_mut();
    };

    let v: serde_json::Value = match serde_json::from_str(opts_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(&format!("invalid JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let seed = seedfaker_core::opts::resolve_seed(v["seed"].as_str());
    let locales = match seedfaker_core::opts::resolve_locales(v["locale"].as_str()) {
        Ok(v) => v,
        Err(e) => {
            set_error(&e);
            return std::ptr::null_mut();
        }
    };
    let since_str = v["since"].as_i64().map(|v| v.to_string());
    let until_str = v["until"].as_i64().map(|v| v.to_string());
    let (tz_offset, since, until) = match seedfaker_core::opts::resolve_time(
        v["tz"].as_str(),
        since_str.as_deref(),
        until_str.as_deref(),
    ) {
        Ok(v) => v,
        Err(e) => {
            set_error(&e);
            return std::ptr::null_mut();
        }
    };

    Box::into_raw(Box::new(SfFaker { seed, locales, record_counter: 0, tz_offset, since, until }))
}

/// Destroy a Faker instance. Must be called for every `sf_create`.
///
/// # Safety
/// `faker` must be null or a valid handle returned by `sf_create`.
#[no_mangle]
pub unsafe extern "C" fn sf_destroy(faker: *mut SfFaker) {
    if !faker.is_null() {
        // SAFETY: caller guarantees valid handle from sf_create, used exactly once.
        drop(unsafe { Box::from_raw(faker) });
    }
}

/// Generate a single field value. Uses domain-hash generation for
/// cross-language parity with PyO3/NAPI bindings.
/// Returns C string or NULL on error. Caller must free with `sf_free`.
///
/// # Safety
/// `faker` must be a valid handle from `sf_create`. `field_name` must be a valid C string.
#[no_mangle]
pub unsafe extern "C" fn sf_field(faker: *mut SfFaker, field_name: *const c_char) -> *mut c_char {
    if faker.is_null() || field_name.is_null() {
        set_error("null pointer");
        return std::ptr::null_mut();
    }
    // SAFETY: null check above, caller guarantees valid handle and C string.
    let sf = unsafe { &mut *faker };
    // SAFETY: null checked above, caller guarantees valid C string.
    let Ok(spec_str) = (unsafe { read_cstr(field_name, "") }) else {
        set_error("field_name is not valid UTF-8");
        return std::ptr::null_mut();
    };

    let (name, modifier, transform, range_spec, _, omit_pct, _) =
        match field::parse_field_spec(spec_str) {
            Ok(v) => v,
            Err(e) => {
                set_error(&e);
                return std::ptr::null_mut();
            }
        };

    let Some(f) = field::lookup(name) else {
        set_error(&format!("unknown field: {name}"));
        return std::ptr::null_mut();
    };

    let range = field::resolve_range(&range_spec, f.name, sf.since, sf.until);
    let spec = pipeline::FieldSpec {
        field: f,
        modifier,
        domain_hash: pipeline::field_domain_hash(sf.seed, f, modifier),
        range,
        transform,
        omit_pct,
    };

    let opts = pipeline::RecordOpts {
        master_seed: sf.seed,
        locales: &sf.locales,
        ctx: seedfaker_core::script::Ctx::None,
        corrupt_rate: None,
        tz_offset_minutes: sf.tz_offset,
        since: sf.since,
        until: sf.until,
    };

    let records = pipeline::generate_records(&opts, &[spec], 1, sf.record_counter);
    sf.record_counter += 1;

    if let Some(vals) = records.into_iter().next() {
        if let Some(val) = vals.into_iter().next() {
            return to_cstring(&val);
        }
    }
    to_cstring("")
}

/// Validate field specs and options without generating data.
/// `opts_json`: `{"fields":["name","email:e164"],"ctx":"strict","corrupt":"low"}`
/// Returns empty string on success. On error, returns NULL and sets error (read with `sf_error`).
///
/// # Safety
/// `faker` must be a valid handle from `sf_create`. `opts_json` must be a valid C string.
#[no_mangle]
pub unsafe extern "C" fn sf_validate(
    faker: *const SfFaker,
    opts_json: *const c_char,
) -> *mut c_char {
    if faker.is_null() || opts_json.is_null() {
        set_error("null pointer");
        return std::ptr::null_mut();
    }
    // SAFETY: null checked above, caller guarantees valid C string.
    let Ok(opts_str) = (unsafe { read_cstr(opts_json, "{}") }) else {
        set_error("opts_json is not valid UTF-8");
        return std::ptr::null_mut();
    };

    let v: serde_json::Value = match serde_json::from_str(opts_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(&format!("invalid JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let Some(fields_arr) = v["fields"].as_array() else {
        set_error("'fields' must be a JSON array");
        return std::ptr::null_mut();
    };
    let mut fields: Vec<String> = Vec::with_capacity(fields_arr.len());
    for (i, x) in fields_arr.iter().enumerate() {
        if let Some(s) = x.as_str() {
            fields.push(String::from(s));
        } else {
            set_error(&format!("fields[{i}] must be a string, got {x}"));
            return std::ptr::null_mut();
        }
    }
    if fields.is_empty() {
        set_error("'fields' must not be empty");
        return std::ptr::null_mut();
    }

    if let Err(e) =
        seedfaker_core::pipeline::validate(&fields, v["ctx"].as_str(), v["corrupt"].as_str())
    {
        set_error(&e);
        return std::ptr::null_mut();
    }

    to_cstring("")
}

/// Generate a single record as JSON object.
/// `opts_json`: `{"fields":["name","email","phone:e164"],"ctx":"strict","corrupt":"low"}`
/// Returns JSON object string or NULL on error. Caller must free with `sf_free`.
///
/// # Safety
/// `faker` must be a valid handle from `sf_create`. `opts_json` must be a valid C string.
#[no_mangle]
pub unsafe extern "C" fn sf_record(faker: *mut SfFaker, opts_json: *const c_char) -> *mut c_char {
    // Delegate to sf_records with n=1, extract first element
    if faker.is_null() || opts_json.is_null() {
        set_error("null pointer");
        return std::ptr::null_mut();
    }
    // SAFETY: null checked above, caller guarantees valid C string.
    let Ok(opts_str) = (unsafe { read_cstr(opts_json, "{}") }) else {
        set_error("opts_json is not valid UTF-8");
        return std::ptr::null_mut();
    };

    // Inject n=1 into the JSON
    let mut v: serde_json::Value = match serde_json::from_str(opts_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(&format!("invalid JSON: {e}"));
            return std::ptr::null_mut();
        }
    };
    v["n"] = serde_json::Value::Number(1.into());
    let patched = v.to_string();

    let Ok(c_patched) = std::ffi::CString::new(patched) else {
        set_error("internal error: JSON contains null byte");
        return std::ptr::null_mut();
    };
    // SAFETY: faker is non-null (checked above), c_patched is a valid CString.
    let result = unsafe { sf_records(faker, c_patched.as_ptr()) };
    if result.is_null() {
        return std::ptr::null_mut();
    }

    // Parse "[{...}]" → extract "{...}"
    // SAFETY: sf_records returned a valid C string we own.
    let arr_str = unsafe { std::ffi::CStr::from_ptr(result) }.to_string_lossy();
    let arr: Vec<serde_json::Value> = match serde_json::from_str(&arr_str) {
        Ok(v) => v,
        Err(e) => {
            // SAFETY: freeing our own allocation.
            unsafe { sf_free(result) };
            set_error(&format!("internal error: {e}"));
            return std::ptr::null_mut();
        }
    };
    // SAFETY: freeing our own allocation.
    unsafe { sf_free(result) };

    if arr.is_empty() {
        set_error("empty result");
        return std::ptr::null_mut();
    }
    match serde_json::to_string(&arr[0]) {
        Ok(s) => to_cstring(&s),
        Err(e) => {
            set_error(&format!("JSON serialization error: {e}"));
            std::ptr::null_mut()
        }
    }
}

struct ParsedRecordOpts {
    fields: Vec<String>,
    n: u64,
    ctx: seedfaker_core::script::Ctx,
    corrupt_rate: Option<f64>,
}

/// Parse fields, n, ctx, corrupt from JSON. Sets error and returns None on failure.
fn parse_record_opts(v: &serde_json::Value) -> Option<ParsedRecordOpts> {
    let Some(fields_arr) = v["fields"].as_array() else {
        set_error("'fields' must be a JSON array");
        return None;
    };
    let mut fields: Vec<String> = Vec::with_capacity(fields_arr.len());
    for (i, x) in fields_arr.iter().enumerate() {
        if let Some(s) = x.as_str() {
            fields.push(String::from(s));
        } else {
            set_error(&format!("fields[{i}] must be a string, got {x}"));
            return None;
        }
    }
    if fields.is_empty() {
        set_error("'fields' must not be empty");
        return None;
    }
    let n = match v.get("n") {
        Some(val) => match val.as_u64() {
            Some(n) if (1..=seedfaker_core::opts::MAX_COUNT).contains(&n) => n,
            Some(n) => {
                set_error(&format!(
                    "n must be between 1 and {}; got {n}",
                    seedfaker_core::opts::MAX_COUNT
                ));
                return None;
            }
            None => {
                set_error(&format!("n must be a positive integer; got {val}"));
                return None;
            }
        },
        None => 1,
    };
    let ctx = match seedfaker_core::opts::resolve_ctx(v["ctx"].as_str()) {
        Ok(v) => v,
        Err(e) => {
            set_error(&e);
            return None;
        }
    };
    let corrupt_rate = match seedfaker_core::opts::resolve_corrupt_rate(v["corrupt"].as_str()) {
        Ok(v) => v,
        Err(e) => {
            set_error(&e);
            return None;
        }
    };
    Some(ParsedRecordOpts { fields, n, ctx, corrupt_rate })
}

/// Generate multiple records as JSON array.
/// `opts_json`: `{"fields":["name","email","phone:e164"],"n":100,"ctx":"strict","corrupt":"low"}`
/// Fields support modifier syntax (e.g. "phone:e164", "amount:usd").
/// Returns JSON string or NULL on error. Caller must free with `sf_free`.
///
/// # Safety
/// `faker` must be a valid handle from `sf_create`. `opts_json` must be a valid C string.
#[no_mangle]
pub unsafe extern "C" fn sf_records(faker: *mut SfFaker, opts_json: *const c_char) -> *mut c_char {
    if faker.is_null() || opts_json.is_null() {
        set_error("null pointer");
        return std::ptr::null_mut();
    }
    // SAFETY: null check above, caller guarantees valid handle.
    let sf = unsafe { &mut *faker };
    // SAFETY: null checked above, caller guarantees valid C string.
    let Ok(opts_str) = (unsafe { read_cstr(opts_json, "{}") }) else {
        set_error("opts_json is not valid UTF-8");
        return std::ptr::null_mut();
    };

    let v: serde_json::Value = match serde_json::from_str(opts_str) {
        Ok(v) => v,
        Err(e) => {
            set_error(&format!("invalid JSON: {e}"));
            return std::ptr::null_mut();
        }
    };

    let Some(p) = parse_record_opts(&v) else {
        return std::ptr::null_mut();
    };

    let opts = pipeline::RecordOpts {
        master_seed: sf.seed,
        locales: &sf.locales,
        ctx: p.ctx,
        corrupt_rate: p.corrupt_rate,
        tz_offset_minutes: sf.tz_offset,
        since: sf.since,
        until: sf.until,
    };

    let (col_names, raw_records) = match seedfaker_core::eval::generate_records_from_specs(
        &p.fields,
        &opts,
        p.n,
        sf.record_counter,
    ) {
        Ok(v) => v,
        Err(e) => {
            set_error(&e);
            return std::ptr::null_mut();
        }
    };
    sf.record_counter += p.n;

    let records: Vec<serde_json::Value> = raw_records
        .into_iter()
        .map(|vals| {
            let mut obj = serde_json::Map::new();
            for (name, val) in col_names.iter().zip(vals) {
                obj.insert(name.clone(), serde_json::Value::String(val));
            }
            serde_json::Value::Object(obj)
        })
        .collect();

    match serde_json::to_string(&records) {
        Ok(json) => to_cstring(&json),
        Err(e) => {
            set_error(&format!("JSON serialization failed: {e}"));
            std::ptr::null_mut()
        }
    }
}

/// List all fields as JSON array.
/// Returns `[{"name":"...","group":"...","description":"..."},...]`.
/// Caller must free with `sf_free`.
#[no_mangle]
pub extern "C" fn sf_fields_json() -> *mut c_char {
    let fields: Vec<serde_json::Value> = field::REGISTRY
        .iter()
        .map(|f| {
            serde_json::json!({
                "name": f.name,
                "group": f.group,
                "description": f.description,
            })
        })
        .collect();
    match serde_json::to_string(&fields) {
        Ok(json) => to_cstring(&json),
        Err(_) => std::ptr::null_mut(),
    }
}

/// Return the algorithm fingerprint. Caller must free with `sf_free`.
#[no_mangle]
pub extern "C" fn sf_fingerprint() -> *mut c_char {
    to_cstring(&seedfaker_core::fingerprint())
}

/// Return build info JSON: {"version":"...","fingerprint":"..."}. Caller must free with `sf_free`.
#[no_mangle]
pub extern "C" fn sf_build_info() -> *mut c_char {
    to_cstring(&seedfaker_core::build_info())
}

/// Free a string returned by any `sf_*` function.
///
/// # Safety
/// `ptr` must be null or a pointer returned by a `sf_*` function.
#[no_mangle]
pub unsafe extern "C" fn sf_free(ptr: *mut c_char) {
    if !ptr.is_null() {
        // SAFETY: caller guarantees ptr was returned by a sf_* function (CString::into_raw).
        drop(unsafe { CString::from_raw(ptr) });
    }
}

/// Return the last error message. Thread-local, valid until the next `sf_*` call.
/// Do NOT free this pointer.
#[no_mangle]
pub extern "C" fn sf_last_error() -> *const c_char {
    LAST_ERROR.with(|e| e.borrow().as_ptr())
}
