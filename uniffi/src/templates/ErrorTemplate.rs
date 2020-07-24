{% match ci.error()%}
    {% when Some with (e) %}
impl From<{{e.name()}}> for ffi_support::ExternError {
    fn from(err: {{e.name()}}) -> ffi_support::ExternError {
        // Errno don't mean anything yet, they just differentiate
        // between the errors.
        // They are in-order, i.e the first variant of the enum has code 1
        match err {
            {%- for value in e.values() %}
            {{ e.name()}}::{{value}} => ffi_support::ExternError::new_error(ffi_support::ErrorCode::new({{loop.index}}), err.to_string()),
            {%- endfor %}
        }
    }
}
    {% when None %}
{% endmatch %}
    