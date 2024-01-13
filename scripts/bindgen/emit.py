if __name__ == '__main__':
    from parse import Signature
else:
    from .parse import Signature

def emit_export(sig: Signature):
    args_in_cpp_syntax = ", ".join(
        ["{ty} {ident}".format(ty = ty, ident = ident) for (ident, ty) in sig.args]
    )
    template = "\n".join([
        r"{output} {ident}({args_in_cpp_syntax}) {{",
        r"    // TODO",
        r"}}"
    ]) + "\n"
    out = template.format(
        args_in_cpp_syntax = args_in_cpp_syntax,
        ident = sig.ident,
        output = sig.output
    )
    return out

def emit_import(sig: Signature):
    arg_de = "\n".join(
        [r"    {ty} arg{id} = do_de<{ty}>(ptr_serialized);".format(ty = ty, id = i)
            for (i, (_ident, ty)) in enumerate(sig.args)]
    )
    arg_names = ", ".join(
        ["arg{id}".format(id = i) for i in range(len(sig.args))]
    )
    template = "\n".join([
        r"BASMCALL size_t basm_import_thunk_{ident}(size_t ptr_serialized) {{",
        r"    static std::vector<uint8_t> s_buf;",
        r"    void free() {{",
        r"        s_buf.clear();",
        r"    }}",
        r"",
        r"    do_de<size_t>(ptr_serialized);",
        arg_de,
        r"    ((void (BASMCALL *)()) do_de<size_t>(ptr_serialized))();",
        r"",
        r"    {output} out = {ident}({arg_names});",
        r"    do_ser_begin(buf, 0);",
        r"    do_ser<{output}>(buf, out);",
        r"    do_ser<size_t>(buf, (size_t) free);",
        r"    do_ser_end(buf, 0);",
        r"    return (size_t) s_buf.as_ptr();",
        r"}}",
    ]) + "\n"
    out = template.format(
        ident = sig.ident,
        arg_de = arg_de,
        arg_names = arg_names,
        output = sig.output
    )
    return out

def emit(sig: Signature):
    assert type(sig) == Signature
    if sig.bindgen_type == Signature.EXPORT:
        return emit_export(sig)
    else:
        assert sig.bindgen_type == Signature.IMPORT
        return emit_import(sig)

if __name__ == '__main__':
    print(emit(Signature("_basm_export_4_init_2_1_t_prim_i32_1_n_prim_i32_prim_unit")))
    print(emit(Signature("_basm_export_4_game_0_prim_unit")))
    print(emit(Signature("_basm_import_5_guess_1_1_b_prim_string_pair_prim_i32_prim_i32")))
    print(emit(Signature("_basm_import_8_test_ptr_2_1_x_prim_ptr_usize_1_y_vec_pair_prim_i8_prim_u64_prim_ptrmut_u8")))