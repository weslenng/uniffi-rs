{# 
// Template to receive calls into rust.
#}

{%- macro to_rs_call(func) -%}
{{ func.name() }}({% call _arg_list_rs_call(func.arguments()) -%})
{%- endmacro -%}

{%- macro to_rs_call_with_prefix(prefix, func) -%}
    {{ func.name() }}(
    {{- prefix }}{% if func.arguments().len() > 0 %}, {% call _arg_list_rs_call(func.arguments()) -%}{% endif -%}
)
{%- endmacro -%}

{%- macro _arg_list_rs_call(args) %}
    {%- for arg in args %}
        {{- arg.name()|lift_rs(arg.type_()) }}
        {%- if !loop.last %}, {% endif %}
    {%- endfor %}
{%- endmacro -%}

{#-
// Arglist as used in the _UniFFILib function declations.
// Note unfiltered name but type_c filters.
-#}
{%- macro arg_list_rs_decl(args) %}
    {%- for arg in args %}
        {{- arg.name() }}: {{ arg.type_()|type_c -}},
    {%- endfor %}
    err: &mut ffi_support::ExternError,
{%- endmacro -%}

{% macro return_type_func(func) %}{% match func.ffi_func().return_type() %}{% when Some with (return_type) %}{{ return_type|ret_type_c }}{%- else -%}(){%- endmatch -%}{%- endmacro -%}

{% macro return_err_type(func) %}{% match func.ffi_func().throws() %}{% when Some with (e) %}{{e}}{% else %}GenericRustError{% endmatch %}{% endmacro %}

{% macro ret(func) %}{% match func.return_type() %}{% when Some with (return_type) %}{{ "_retval"|lower_rs(return_type) }}{% else %}_retval{% endmatch %}{% endmacro %}

{% macro try_retval(func) %}
{% match func.throws() %}{% when Some with (e) %}
            let _retval = _retval?;
{% else %}
{% endmatch %}
{% endmacro %}
