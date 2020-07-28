{#
// For each top-level function declared in the IDL, we assume the caller has provided a corresponding
// rust function of the same name. We provide a `pub extern "C"` wrapper that does type conversions to
// send data across the FFI, which will fail to compile if the provided function does not match what's
// specified in the IDL.    
#}

#[no_mangle]
pub extern "C" fn {{ func.ffi_func().name() }}(
    {% call rs::arg_list_rs_decl(func.ffi_func()) %}
) -> {% call rs::return_type_func(func) %} {
    log::debug!("{{ func.ffi_func().name() }}");
    ffi_support::call_with_result(err, || -> Result<{% call macros::return_type_func(func) %}, {% call macros::return_err_type(func) %}> {
        // If the provided function does not match the signature specified in the IDL
        // then this attempt to cal it will not compile, and will give guideance as to why.
        let _retval = {% call rs::to_rs_call(func) %};
        {% call macros::try_retval(func) %}
        Ok({% call macros::ret(func) %})
    })
}
