def convert_type(x):
    conv_dict = dict()
    base_types = {
        "i8": "int8_t",
        "i16": "int16_t",
        "i32": "int32_t",
        "i64": "int64_t",
        "isize": "intptr_t",
        "u8": "uint8_t",
        "u16": "uint16_t",
        "u32": "uint32_t",
        "u64": "uint64_t",
        "usize": "size_t"
    }
    for k, v in base_types.items():
        conv_dict[k] = v
        conv_dict["* const {0}".format(k)] = "const {0} *".format(v)
        conv_dict["* mut {0}".format(k)] = "{0} *".format(v)
    assert x in conv_dict
    return conv_dict[x]

def convert_arg(arg):
    arg = [x.strip() for x in arg.split(":")]
    assert len(arg) == 2
    arg_name, arg_type = arg[0], convert_type(arg[1])
    return (arg_name, arg_type)

def synthesize(e_name, e_offset):
    if len(e_name) not in [2, 3]:
        raise Exception("len(e_name) must be 2 or 3, but got len(e_name) == {0}".format(len(e_name)))

    # method name
    method_name = e_name[0]

    # args
    args = e_name[1]
    args = [x.strip() for x in args.split(",")]
    args = [convert_arg(x) for x in args]
    args_fn_body = ", ".join(["{1} {0}".format(x[0], x[1]) for x in args])
    args_fn_decl = ", ".join(["{0}".format(x[1]) for x in args])
    args_fn_call = ", ".join(["{0}".format(x[0]) for x in args])

    # return type
    return_type = "void"
    if len(e_name) == 3:
        return_type = convert_type(e_name[2])

    # Substitute the informations into the template
    # Note: The loader template should define the macro
    #    BASM_LOADER_IMAGEBASE, which loads the binary only once
    #    and returns the in-memory imagebase of the binary.
    #    It is best implemented as a function call, like `get_imagebase()`.
    template = "\n".join([
        "#if defined(BASM_LOADER_IMAGEBASE)",
        "{4} {0}({1}) {{",
        "    return (({4} (BASMCALL *)({2}))(BASM_LOADER_IMAGEBASE + {5}))({3});",
        "}}",
        "#endif"
    ])
    return template.format(method_name, args_fn_body, args_fn_decl, args_fn_call, return_type, e_offset)