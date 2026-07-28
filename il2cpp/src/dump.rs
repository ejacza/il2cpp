use crate::api::Il2CppApi;
use crate::tabledefs as td;
use core::ffi::c_void;
use std::ffi::CStr;
use std::fmt::Write;

fn cstr<'a>(p: *const u8) -> &'a str {
    if p.is_null() {
        ""
    } else {
        unsafe { CStr::from_ptr(p as *const i8).to_str().unwrap_or("") }
    }
}

fn modifier(flags: u32) -> String {
    let mut s = String::new();
    match flags & td::METHOD_ATTRIBUTE_MEMBER_ACCESS_MASK {
        td::METHOD_ATTRIBUTE_PRIVATE => s.push_str("private "),
        td::METHOD_ATTRIBUTE_PUBLIC => s.push_str("public "),
        td::METHOD_ATTRIBUTE_FAMILY => s.push_str("protected "),
        td::METHOD_ATTRIBUTE_ASSEM | td::METHOD_ATTRIBUTE_FAM_AND_ASSEM => s.push_str("internal "),
        td::METHOD_ATTRIBUTE_FAM_OR_ASSEM => s.push_str("protected internal "),
        _ => {}
    }
    if flags & td::METHOD_ATTRIBUTE_STATIC != 0 {
        s.push_str("static ");
    }
    if flags & td::METHOD_ATTRIBUTE_ABSTRACT != 0 {
        s.push_str("abstract ");
        if (flags & td::METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK) == td::METHOD_ATTRIBUTE_REUSE_SLOT {
            s.push_str("override ");
        }
    } else if flags & td::METHOD_ATTRIBUTE_FINAL != 0 {
        if (flags & td::METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK) == td::METHOD_ATTRIBUTE_REUSE_SLOT {
            s.push_str("sealed override ");
        }
    } else if flags & td::METHOD_ATTRIBUTE_VIRTUAL != 0 {
        if (flags & td::METHOD_ATTRIBUTE_VTABLE_LAYOUT_MASK) == td::METHOD_ATTRIBUTE_NEW_SLOT {
            s.push_str("virtual ");
        } else {
            s.push_str("override ");
        }
    }
    if flags & td::METHOD_ATTRIBUTE_PINVOKE_IMPL != 0 {
        s.push_str("extern ");
    }
    s
}

fn is_byref(api: &Il2CppApi, t: *mut c_void) -> bool {
    if t.is_null() {
        return false;
    }
    let b = unsafe { ((*(t as *const u8).add(11)) >> 6) & 1 != 0 };
    if let Some(f) = api.type_is_byref {
        b || unsafe { f(t) }
    } else {
        b
    }
}

fn dump_method(api: &Il2CppApi, klass: *const c_void) -> String {
    let mut out = String::new();
    let mut iter = std::ptr::null_mut();
    let mut first = true;
    loop {
        let method = match api.class_get_methods {
            Some(f) => unsafe { f(klass, &mut iter) },
            None => break,
        };
        if method.is_null() {
            break;
        }
        if first {
            out.push_str(
                "
	// Methods
",
            );
            first = false;
        } else {
            out.push('\n');
        }
        let fp = unsafe { *(method as *const *mut c_void) };
        if !fp.is_null() {
            let _ = write!(
                out,
                "	// RVA: 0x{:x} VA: 0x{:x}
	",
                fp as u64, fp as u64
            );
        } else {
            out.push_str(
                "	// RVA: 0x VA: 0x0
	",
            );
        }
        let mut iflags: u32 = 0;
        let flags = match api.method_get_flags {
            Some(f) => unsafe { f(method, &mut iflags) },
            None => 0,
        };
        out.push_str(&modifier(flags));
        let rt = match api.method_get_return_type {
            Some(f) => unsafe { f(method) },
            None => std::ptr::null_mut(),
        };
        if is_byref(api, rt) {
            out.push_str("ref ");
        }
        let rc = match api.class_from_type {
            Some(f) => unsafe { f(rt) },
            None => std::ptr::null_mut(),
        };
        let rn = match api.class_get_name {
            Some(f) => unsafe { f(rc as *const c_void) },
            None => std::ptr::null(),
        };
        let mn = match api.method_get_name {
            Some(f) => unsafe { f(method) },
            None => std::ptr::null(),
        };
        let _ = write!(out, "{} {}(", cstr(rn), cstr(mn));
        let pc = match api.method_get_param_count {
            Some(f) => unsafe { f(method) },
            None => 0,
        };
        for i in 0..pc {
            let param = match api.method_get_param {
                Some(f) => unsafe { f(method, i) },
                None => continue,
            };
            let attrs = unsafe { *(param as *const u16).add(4) as u32 };
            if is_byref(api, param) {
                if attrs & td::PARAM_ATTRIBUTE_OUT != 0 && attrs & td::PARAM_ATTRIBUTE_IN == 0 {
                    out.push_str("out ");
                } else if attrs & td::PARAM_ATTRIBUTE_IN != 0
                    && attrs & td::PARAM_ATTRIBUTE_OUT == 0
                {
                    out.push_str("in ");
                } else {
                    out.push_str("ref ");
                }
            } else {
                if attrs & td::PARAM_ATTRIBUTE_IN != 0 {
                    out.push_str("[In] ");
                }
                if attrs & td::PARAM_ATTRIBUTE_OUT != 0 {
                    out.push_str("[Out] ");
                }
            }
            let pclass = match api.class_from_type {
                Some(f) => unsafe { f(param) },
                None => std::ptr::null_mut(),
            };
            let pn = match api.method_get_param_name {
                Some(f) => unsafe { f(method, i) },
                None => std::ptr::null(),
            };
            let pc_name = match api.class_get_name {
                Some(f) => unsafe { f(pclass as *const c_void) },
                None => std::ptr::null(),
            };
            let _ = write!(out, "{} {} ", cstr(pc_name), cstr(pn));
        }
        if pc > 0 {
            out.pop();
            out.pop();
        }
        out.push_str(
            ") { }
",
        );
    }
    out
}

fn dump_property(api: &Il2CppApi, klass: *const c_void) -> String {
    let mut out = String::new();
    let mut iter = std::ptr::null_mut();
    let mut first = true;
    loop {
        let prop_const = match api.class_get_properties {
            Some(f) => unsafe { f(klass, &mut iter) },
            None => break,
        };
        if prop_const.is_null() {
            break;
        }
        let getter = match api.property_get_get_method {
            Some(f) => unsafe { f(prop_const) },
            None => std::ptr::null_mut(),
        };
        let setter = match api.property_get_set_method {
            Some(f) => unsafe { f(prop_const) },
            None => std::ptr::null_mut(),
        };
        let pn = match api.property_get_name {
            Some(f) => unsafe { f(prop_const) },
            None => std::ptr::null(),
        };
        if first {
            out.push_str(
                "
	// Properties
",
            );
            first = false;
        }
        out.push('\t');
        let mut iflags: u32 = 0;
        let pclass;
        if !getter.is_null() {
            let gf = match api.method_get_flags {
                Some(f) => unsafe { f(getter, &mut iflags) },
                None => 0,
            };
            out.push_str(&modifier(gf));
            let rt = match api.method_get_return_type {
                Some(f) => unsafe { f(getter) },
                None => std::ptr::null_mut(),
            };
            pclass = match api.class_from_type {
                Some(f) => unsafe { f(rt) },
                None => std::ptr::null_mut(),
            };
        } else if !setter.is_null() {
            let sf = match api.method_get_flags {
                Some(f) => unsafe { f(setter, &mut iflags) },
                None => 0,
            };
            out.push_str(&modifier(sf));
            let param = match api.method_get_param {
                Some(f) => unsafe { f(setter, 0) },
                None => std::ptr::null_mut(),
            };
            pclass = match api.class_from_type {
                Some(f) => unsafe { f(param) },
                None => std::ptr::null_mut(),
            };
        } else {
            pclass = std::ptr::null_mut();
        }
        if !pclass.is_null() {
            let pc_name = match api.class_get_name {
                Some(f) => unsafe { f(pclass as *const c_void) },
                None => std::ptr::null(),
            };
            let _ = write!(out, "{} {} {{ ", cstr(pc_name), cstr(pn));
            if !getter.is_null() {
                out.push_str("get; ");
            }
            if !setter.is_null() {
                out.push_str("set; ");
            }
            out.push_str(
                "}
",
            );
        } else if !pn.is_null() {
            let _ = write!(
                out,
                " // unknown property {}
",
                cstr(pn)
            );
        }
    }
    out
}

fn dump_field(api: &Il2CppApi, klass: *const c_void) -> String {
    let mut out = String::new();
    let is_enum = match api.class_is_enum {
        Some(f) => unsafe { f(klass) },
        None => false,
    };
    let mut iter = std::ptr::null_mut();
    let mut first = true;
    loop {
        let field = match api.class_get_fields {
            Some(f) => unsafe { f(klass, &mut iter) },
            None => break,
        };
        if field.is_null() {
            break;
        }
        if first {
            out.push_str(
                "
	// Fields
",
            );
            first = false;
        }
        out.push('\t');
        let attrs = match api.field_get_flags {
            Some(f) => unsafe { f(field) },
            None => 0,
        };
        match attrs & td::FIELD_ATTRIBUTE_FIELD_ACCESS_MASK {
            td::FIELD_ATTRIBUTE_PRIVATE => out.push_str("private "),
            td::FIELD_ATTRIBUTE_PUBLIC => out.push_str("public "),
            td::FIELD_ATTRIBUTE_FAMILY => out.push_str("protected "),
            td::FIELD_ATTRIBUTE_ASSEMBLY | td::FIELD_ATTRIBUTE_FAM_AND_ASSEM => {
                out.push_str("internal ")
            }
            td::FIELD_ATTRIBUTE_FAM_OR_ASSEM => out.push_str("protected internal "),
            _ => {}
        }
        if attrs & td::FIELD_ATTRIBUTE_LITERAL != 0 {
            out.push_str("const ");
        } else {
            if attrs & td::FIELD_ATTRIBUTE_STATIC != 0 {
                out.push_str("static ");
            }
            if attrs & td::FIELD_ATTRIBUTE_INIT_ONLY != 0 {
                out.push_str("readonly ");
            }
        }
        let ft = match api.field_get_type {
            Some(f) => unsafe { f(field) },
            None => std::ptr::null_mut(),
        };
        let fclass = match api.class_from_type {
            Some(f) => unsafe { f(ft) },
            None => std::ptr::null_mut(),
        };
        let fc_name = match api.class_get_name {
            Some(f) => unsafe { f(fclass as *const c_void) },
            None => std::ptr::null(),
        };
        let fn_name = match api.field_get_name {
            Some(f) => unsafe { f(field) },
            None => std::ptr::null(),
        };
        let _ = write!(out, "{} {}", cstr(fc_name), cstr(fn_name));
        if attrs & td::FIELD_ATTRIBUTE_LITERAL != 0 && is_enum {
            let mut val: u64 = 0;
            if let Some(f) = api.field_static_get_value {
                unsafe { f(field, &mut val as *mut u64 as *mut c_void) };
            }
            let _ = write!(out, " = {}", val);
        }
        let off = match api.field_get_offset {
            Some(f) => unsafe { f(field) },
            None => 0,
        };
        let _ = write!(
            out,
            "; // 0x{:x}
",
            off
        );
    }
    out
}

fn dump_type(api: &Il2CppApi, t: *mut c_void) -> String {
    let mut out = String::new();
    let klass = match api.class_from_type {
        Some(f) => unsafe { f(t) },
        None => return String::new(),
    };
    let ns = match api.class_get_namespace {
        Some(f) => unsafe { f(klass as *const c_void) },
        None => std::ptr::null(),
    };
    let _ = write!(
        out,
        "
// Namespace: {}
",
        cstr(ns)
    );
    let flags = match api.class_get_flags {
        Some(f) => unsafe { f(klass as *const c_void) },
        None => 0,
    };
    if flags & td::TYPE_ATTRIBUTE_SERIALIZABLE != 0 {
        out.push_str(
            "[Serializable]
",
        );
    }
    let is_vt = match api.class_is_valuetype {
        Some(f) => unsafe { f(klass) },
        None => false,
    };
    let is_en = match api.class_is_enum {
        Some(f) => unsafe { f(klass) },
        None => false,
    };
    match flags & td::TYPE_ATTRIBUTE_VISIBILITY_MASK {
        td::TYPE_ATTRIBUTE_PUBLIC | td::TYPE_ATTRIBUTE_NESTED_PUBLIC => out.push_str("public "),
        td::TYPE_ATTRIBUTE_NOT_PUBLIC
        | td::TYPE_ATTRIBUTE_NESTED_FAM_AND_ASSEM
        | td::TYPE_ATTRIBUTE_NESTED_ASSEMBLY => out.push_str("internal "),
        td::TYPE_ATTRIBUTE_NESTED_PRIVATE => out.push_str("private "),
        td::TYPE_ATTRIBUTE_NESTED_FAMILY => out.push_str("protected "),
        td::TYPE_ATTRIBUTE_NESTED_FAM_OR_ASSEM => out.push_str("protected internal "),
        _ => {}
    }
    if flags & td::TYPE_ATTRIBUTE_ABSTRACT != 0 && flags & td::TYPE_ATTRIBUTE_SEALED != 0 {
        out.push_str("static ");
    } else if (flags & td::TYPE_ATTRIBUTE_INTERFACE) == 0
        && flags & td::TYPE_ATTRIBUTE_ABSTRACT != 0
    {
        out.push_str("abstract ");
    } else if !is_vt && !is_en && flags & td::TYPE_ATTRIBUTE_SEALED != 0 {
        out.push_str("sealed ");
    }
    if flags & td::TYPE_ATTRIBUTE_INTERFACE != 0 {
        out.push_str("interface ");
    } else if is_en {
        out.push_str("enum ");
    } else if is_vt {
        out.push_str("struct ");
    } else {
        out.push_str("class ");
    }
    let kn = match api.class_get_name {
        Some(f) => unsafe { f(klass as *const c_void) },
        None => std::ptr::null(),
    };
    out.push_str(cstr(kn));
    let parent = match api.class_get_parent {
        Some(f) => unsafe { f(klass as *const c_void) },
        None => std::ptr::null_mut(),
    };
    let mut extends: Vec<String> = Vec::new();
    if !is_vt && !is_en && !parent.is_null() {
        let pt = match api.class_get_type {
            Some(f) => unsafe { f(parent as *const c_void) },
            None => std::ptr::null_mut(),
        };
        if !pt.is_null() && unsafe { *(pt as *const u8).add(10) != 0x1c } {
            let pn = match api.class_get_name {
                Some(f) => unsafe { f(parent as *const c_void) },
                None => std::ptr::null(),
            };
            extends.push(cstr(pn).to_string());
        }
    }
    let mut iter = std::ptr::null_mut();
    loop {
        let itf = match api.class_get_interfaces {
            Some(f) => unsafe { f(klass as *const c_void, &mut iter) },
            None => break,
        };
        if itf.is_null() {
            break;
        }
        let iname = match api.class_get_name {
            Some(f) => unsafe { f(itf as *const c_void) },
            None => std::ptr::null(),
        };
        extends.push(cstr(iname).to_string());
    }
    if !extends.is_empty() {
        out.push_str(" : ");
        out.push_str(&extends[0]);
        for e in extends.iter().skip(1) {
            let _ = write!(out, ", {}", e);
        }
    }
    let fd = dump_field(api, klass as *const c_void);
    let pd = dump_property(api, klass as *const c_void);
    let md = dump_method(api, klass as *const c_void);
    if fd.is_empty() && pd.is_empty() && md.is_empty() {
        return String::new();
    }
    out.push_str(
        "
{",
    );
    out.push_str(&fd);
    out.push_str(&pd);
    out.push_str(&md);
    out.push_str(
        "}
",
    );
    out
}

pub fn run_dump(api: &Il2CppApi) {
    let domain = match api.domain_get {
        Some(f) => unsafe { f() },
        None => {
            return;
        }
    };
    if domain.is_null() {
        return;
    }
    if let Some(f) = api.thread_attach {
        unsafe {
            f(domain);
        }
    }
    let mut asm_count: usize = 0;
    let assemblies = match api.domain_get_assemblies {
        Some(f) => unsafe { f(domain, &mut asm_count) },
        None => {
            return;
        }
    };
    if asm_count == 0 {
        return;
    }
    let mut outputs: Vec<String> = Vec::new();
    for i in 0..asm_count {
        let asm = unsafe { *assemblies.add(i) };
        let image = match api.assembly_get_image {
            Some(f) => unsafe { f(asm) },
            None => continue,
        };
        let iname = match api.image_get_name {
            Some(f) => unsafe { f(image) },
            None => {
                continue;
            }
        };
        let image_str = format!(
            "
// Dll : {}",
            cstr(iname)
        );
        let cc = match api.image_get_class_count {
            Some(f) => unsafe { f(image) },
            None => 0,
        };
        for j in 0..cc {
            let c = match api.image_get_class {
                Some(f) => unsafe { f(image, j) },
                None => continue,
            };
            let t = match api.class_get_type {
                Some(f) => unsafe { f(c as *const c_void) },
                None => continue,
            };
            let dt = dump_type(api, t);
            if !dt.is_empty() {
                outputs.push(format!("{}{}", image_str, dt));
            }
        }
    }
    let out_path = "/sdcard/dump.cs";
    let mut content = String::new();
    for o in &outputs {
        content.push_str(o);
    }
    let _ = std::fs::write(out_path, &content);
}
