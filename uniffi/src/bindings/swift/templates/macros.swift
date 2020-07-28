{# 
// Template to call into rust. Used in several places.
// Variable names in `arg_list_decl` should match up with arg lists
// passed to rust via `_arg_list_rs_call` (we use  `var_name_swift` in `lower_swift`)
#}

{%- macro to_rs_call(func) -%}
try rustCallWith{% match cons.ffi_func().throws() %}{% when Some with (e) %}{{e}}{% else %}{% endmatch %} { err  in
    {{ func.ffi_func().name() }}({% call _arg_list_rs_call(func.arguments()) -%}{% if func.arguments().len() > 0 %},{% endif %}err)
{%- endmacro -%}

{%- macro to_rs_call_with_prefix(prefix, func) -%}
try rustCallWith{% match cons.ffi_func().throws() %}{% when Some with (e) %}{{e}}{% else %}{% endmatch %} { err  in
    {{ func.ffi_func().name() }}(
        {{- prefix }}{% if func.arguments().len() > 0 %}, {% call _arg_list_rs_call(func.arguments()) -%}{% endif -%}{% if func.arguments().len() > 0 %},{% endif %}err
)
{%- endmacro -%}

{%- macro _arg_list_rs_call(args) %}
    {%- for arg in args %}
        {{- arg.name()|lower_swift(arg.type_()) }},
    {%- endfor %}
{%- endmacro -%}

{#-
// Arglist as used in Swift declarations of methods, functions and constructors.
// Note the var_name_swift and decl_swift filters.
-#}

{% macro arg_list_decl(args) %}
    {%- for arg in args -%}
        {{ arg.name()|var_name_swift }}: {{ arg.type_()|decl_swift -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{#-
// Arglist as used in the _UniFFILib function declations.
// Note unfiltered name but type_c filters.
-#}
{%- macro arg_list_rs_decl(func) %}
    {%- for arg in func.arguments() %}
        {{- arg.type_()|decl_c }} {{ arg.name() -}}
        {% if loop.last && !func.has_error() %}{% else %},{% endif %}
    {%- endfor %}
    {% match func.throws() %}{% when Some with (e) %}Native{{e}} *_Nonnull out_err{% else %}{% endmatch %}
{%- endmacro -%}
