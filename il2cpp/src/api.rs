use core::ffi::c_void;

extern "C" {
    fn xdl_sym(handle: *mut c_void, symbol: *const u8, symbol_size: *mut usize) -> *mut c_void;
}

type P = *mut c_void;

fn sym<T>(h: *mut c_void, name: &[u8]) -> Option<T> {
    let ptr = unsafe { xdl_sym(h, name.as_ptr(), std::ptr::null_mut()) };
    if ptr.is_null() {
        None
    } else {
        Some(unsafe { std::mem::transmute_copy::<*mut c_void, T>(&ptr) })
    }
}

#[allow(dead_code)]
pub struct Il2CppApi {
    pub domain_get: Option<unsafe extern "C" fn() -> P>,
    pub domain_get_assemblies: Option<unsafe extern "C" fn(P, *mut usize) -> *mut P>,
    pub assembly_get_image: Option<unsafe extern "C" fn(P) -> P>,
    pub image_get_name: Option<unsafe extern "C" fn(P) -> *const u8>,
    pub image_get_class_count: Option<unsafe extern "C" fn(P) -> usize>,
    pub image_get_class: Option<unsafe extern "C" fn(P, usize) -> P>,
    pub class_get_name: Option<unsafe extern "C" fn(*const c_void) -> *const u8>,
    pub class_get_namespace: Option<unsafe extern "C" fn(*const c_void) -> *const u8>,
    pub class_get_parent: Option<unsafe extern "C" fn(*const c_void) -> P>,
    pub class_get_type: Option<unsafe extern "C" fn(*const c_void) -> P>,
    pub class_get_flags: Option<unsafe extern "C" fn(*const c_void) -> u32>,
    pub class_is_valuetype: Option<unsafe extern "C" fn(*const c_void) -> bool>,
    pub class_is_enum: Option<unsafe extern "C" fn(*const c_void) -> bool>,
    pub class_is_interface: Option<unsafe extern "C" fn(*const c_void) -> bool>,
    pub class_is_abstract: Option<unsafe extern "C" fn(*const c_void) -> bool>,
    pub class_from_type: Option<unsafe extern "C" fn(P) -> P>,
    pub class_get_methods: Option<unsafe extern "C" fn(*const c_void, *mut P) -> P>,
    pub class_get_fields: Option<unsafe extern "C" fn(*const c_void, *mut P) -> P>,
    pub class_get_properties: Option<unsafe extern "C" fn(*const c_void, *mut P) -> P>,
    pub class_get_interfaces: Option<unsafe extern "C" fn(*const c_void, *mut P) -> P>,
    pub is_vm_thread: Option<unsafe extern "C" fn(P) -> bool>,
    pub thread_attach: Option<unsafe extern "C" fn(P) -> P>,
    pub method_get_name: Option<unsafe extern "C" fn(P) -> *const u8>,
    pub method_get_return_type: Option<unsafe extern "C" fn(P) -> P>,
    pub method_get_param: Option<unsafe extern "C" fn(P, u32) -> P>,
    pub method_get_param_count: Option<unsafe extern "C" fn(P) -> u32>,
    pub method_get_param_name: Option<unsafe extern "C" fn(P, u32) -> *const u8>,
    pub method_get_flags: Option<unsafe extern "C" fn(P, *mut u32) -> u32>,
    pub field_get_name: Option<unsafe extern "C" fn(P) -> *const u8>,
    pub field_get_type: Option<unsafe extern "C" fn(P) -> P>,
    pub field_get_flags: Option<unsafe extern "C" fn(P) -> u32>,
    pub field_get_offset: Option<unsafe extern "C" fn(P) -> usize>,
    pub field_static_get_value: Option<unsafe extern "C" fn(P, P)>,
    pub property_get_name: Option<unsafe extern "C" fn(P) -> *const u8>,
    pub property_get_get_method: Option<unsafe extern "C" fn(P) -> P>,
    pub property_get_set_method: Option<unsafe extern "C" fn(P) -> P>,
    pub type_is_byref: Option<unsafe extern "C" fn(P) -> bool>,
}

pub fn init_api(handle: *mut c_void) -> Il2CppApi {
    Il2CppApi {
        domain_get: sym(handle, b"il2cpp_domain_get\0"),
        domain_get_assemblies: sym(handle, b"il2cpp_domain_get_assemblies\0"),
        assembly_get_image: sym(handle, b"il2cpp_assembly_get_image\0"),
        image_get_name: sym(handle, b"il2cpp_image_get_name\0"),
        image_get_class_count: sym(handle, b"il2cpp_image_get_class_count\0"),
        image_get_class: sym(handle, b"il2cpp_image_get_class\0"),
        class_get_name: sym(handle, b"il2cpp_class_get_name\0"),
        class_get_namespace: sym(handle, b"il2cpp_class_get_namespace\0"),
        class_get_parent: sym(handle, b"il2cpp_class_get_parent\0"),
        class_get_type: sym(handle, b"il2cpp_class_get_type\0"),
        class_get_flags: sym(handle, b"il2cpp_class_get_flags\0"),
        class_is_valuetype: sym(handle, b"il2cpp_class_is_valuetype\0"),
        class_is_enum: sym(handle, b"il2cpp_class_is_enum\0"),
        class_is_interface: sym(handle, b"il2cpp_class_is_interface\0"),
        class_is_abstract: sym(handle, b"il2cpp_class_is_abstract\0"),
        class_from_type: sym(handle, b"il2cpp_class_from_type\0"),
        class_get_methods: sym(handle, b"il2cpp_class_get_methods\0"),
        class_get_fields: sym(handle, b"il2cpp_class_get_fields\0"),
        class_get_properties: sym(handle, b"il2cpp_class_get_properties\0"),
        class_get_interfaces: sym(handle, b"il2cpp_class_get_interfaces\0"),
        is_vm_thread: sym(handle, b"il2cpp_is_vm_thread\0"),
        thread_attach: sym(handle, b"il2cpp_thread_attach\0"),
        method_get_name: sym(handle, b"il2cpp_method_get_name\0"),
        method_get_return_type: sym(handle, b"il2cpp_method_get_return_type\0"),
        method_get_param: sym(handle, b"il2cpp_method_get_param\0"),
        method_get_param_count: sym(handle, b"il2cpp_method_get_param_count\0"),
        method_get_param_name: sym(handle, b"il2cpp_method_get_param_name\0"),
        method_get_flags: sym(handle, b"il2cpp_method_get_flags\0"),
        field_get_name: sym(handle, b"il2cpp_field_get_name\0"),
        field_get_type: sym(handle, b"il2cpp_field_get_type\0"),
        field_get_flags: sym(handle, b"il2cpp_field_get_flags\0"),
        field_get_offset: sym(handle, b"il2cpp_field_get_offset\0"),
        field_static_get_value: sym(handle, b"il2cpp_field_static_get_value\0"),
        property_get_name: sym(handle, b"il2cpp_property_get_name\0"),
        property_get_get_method: sym(handle, b"il2cpp_property_get_get_method\0"),
        property_get_set_method: sym(handle, b"il2cpp_property_get_set_method\0"),
        type_is_byref: sym(handle, b"il2cpp_type_is_byref\0"),
    }
}
