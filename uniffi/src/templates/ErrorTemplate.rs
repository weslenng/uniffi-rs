{#
    // For each error declared in the IDL, using the [Error] attribute, we assume the caller as provided a corresponding
    // rust Error enum with the same name. We provide the traits for sending it across the FFI, which will fail to
    // compile if the provided enum has a different shape to the one declared in the IDL.
    //
    // Here we define the neccessary converstion to allow the error to propegate through the FFI as an error.
    // The consumer also has the option of using the Enum itself as an argument/return type. But that has nothing
    // to do with the fact that it is an error, and that logic is defined in the error template.
#}
impl From<{{e.name()}}> for ffi_support::ExternError {
    fn from(err: {{e.name()}}) -> ffi_support::ExternError {
        // Errno don't mean anything yet, they just differentiate
        // between the errors.
        // They are in-order, i.e the first variant of the enum has code 1
        match err {
            {%- for value in e.values() %}
            {{ e.name()}}::{{value}} => ffi_support::ExternError::new_error(ffi_support::ErrorCode::new({{ loop.index }}), err.to_string()),
            {%- endfor %}
        }
    }
}