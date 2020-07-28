{# 
// Template to call into rust. Used in several places.
// Variable names in `arg_list_decl` should match up with arg lists
// passed to rust via `_arg_list_rs_call` (we use  `var_name_kt` in `lower_kt`)
#}

{%- macro to_rs_call(func) -%}
 rustCallWith{% match func.ffi_func().throws() %}{% when Some with (e) %}{{e}}{% else %}{% endmatch %} { e -> 
    _UniFFILib.INSTANCE.{{ func.ffi_func().name() }}({% call _arg_list_rs_call(func.arguments()) -%})
 }
{%- endmacro -%}

{%- macro to_rs_call_with_prefix(prefix, func) -%}
 rustCallWith{% match func.ffi_func().throws() %}{% when Some with (e) %}{{e}}{% else %}{% endmatch %} { e -> 
    _UniFFILib.INSTANCE.{{ func.ffi_func().name() }}(
        {{- prefix }}{% if func.arguments().len() > 0 %}, {% call _arg_list_rs_call(func.arguments()) -%}{% endif -%}
 }
)
{%- endmacro -%}


//Note the `e` here references the `e` defined in the to_rs_call macros
// And the need to match in name.
{%- macro _arg_list_rs_call(func) %}
    {%- for arg in func.arguments() %}
        {{- arg.name()|lower_kt(arg.type_()) }}
        {%- if !loop.last  && !func.has_error() %}, {% endif %}
        {% match func.throws() %}{% when Some with (err) %}e{% else %}{% endmatch %}
    {%- endfor %}
{%- endmacro -%}

{#-
// Arglist as used in kotlin declarations of methods, functions and constructors.
// Note the var_name_kt and type_kt filters.
-#}

{% macro arg_list_decl(args) %}
    {%- for arg in args -%}
        {{ arg.name()|var_name_kt }}: {{ arg.type_()|type_kt -}}
        {%- if !loop.last %}, {% endif -%}
    {%- endfor %}
{%- endmacro %}

{#-
// Arglist as used in the _UniFFILib function declations.
// Note unfiltered name but type_c filters.
-#}
{%- macro arg_list_rs_decl(func) %}
    {%- for arg in func.arguments() %}
        {{- arg.name() }}: {{ arg.type_()|type_c -}}
        {%- if !loop.last && !func.has_error() %}, {% endif %}
        {% match func.throws() %}{% when Some with (e) %}error: {{e}}.ByReference{% else %}{% endmatch %}
    {%- endfor %}
{%- endmacro -%}
