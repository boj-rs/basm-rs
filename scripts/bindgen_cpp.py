import re

def emit_thunk_for_plain(arg_name, arg_type, is_output_type):
    if not is_output_type:
        fn_decl, fn_ptr_decl, prologue, invocation = [], [], [], []
        fn_decl.append("{1} {0}".format(arg_name, arg_type))
        fn_ptr_decl.append("{0}".format(arg_type))
        invocation.append("{0}".format(arg_name))
        return (fn_decl, fn_ptr_decl, prologue, invocation)
    else:
        return arg_type

def emit_thunk_for_vector(arg_name, arg_type, is_output_type):
    assert is_output_type == False, "Returning a vector is not yet implemented."
    fn_decl, fn_ptr_decl, prologue, invocation = [], [], [], []

    p = re.compile('\<([A-Za-z0-9_]+)\>')
    fn_decl.append("{1} {0}".format(arg_name, arg_type))
    fn_ptr_decl.append("{1}{0} *".format(p.search(arg_type).group(1), "const " if "const" in arg_type else ""))
    fn_ptr_decl.append("size_t")
    invocation.append("{0}.data()".format(arg_name))
    invocation.append("{0}.size()".format(arg_name))
    return (fn_decl, fn_ptr_decl, prologue, invocation)

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
        conv_dict[k] = (v, emit_thunk_for_plain)
        conv_dict["* const {0}".format(k)] = ("const {0} *".format(v), emit_thunk_for_plain)
        conv_dict["* mut {0}".format(k)] = ("{0} *".format(v), emit_thunk_for_plain)
        conv_dict["Vec :: < {0} >".format(k)] = ("std::vector<{0}>".format(v), emit_thunk_for_vector)
        conv_dict["& Vec :: < {0} >".format(k)] = ("const std::vector<{0}> &".format(v), emit_thunk_for_vector)
        conv_dict["& mut Vec :: < {0} >".format(k)] = ("std::vector<{0}> &".format(v), emit_thunk_for_vector)
    assert x in conv_dict
    return conv_dict[x]

def convert_arg(arg):
    arg = [x.strip() for x in arg.split(" ")]
    assert arg.count(":") == 1
    idx = arg.index(":")
    arg_name, arg_type = arg[:idx], arg[idx+1:]
    arg_name = "".join(arg_name)
    arg_type, emit_thunk = convert_type(" ".join(arg_type))
    return (arg_name, arg_type, emit_thunk)

def synthesize(e_name, e_offset):
    if len(e_name) not in [2, 3]:
        raise Exception("len(e_name) must be 2 or 3, but got len(e_name) == {0}".format(len(e_name)))

    # method name
    method_name = e_name[0]

    # args
    args = e_name[1]
    args = [x.strip() for x in args.split(",")]
    args = [convert_arg(x) for x in args]
    fn_decl, fn_ptr_decl, prologue, invocation = [], [], [], []
    for arg_name, arg_type, emit_thunk in args:
        fn_decl_, fn_ptr_decl_, prologue_, invocation_ = emit_thunk(arg_name, arg_type, False)
        fn_decl.extend(fn_decl_)
        fn_ptr_decl.extend(fn_ptr_decl_)
        prologue.extend(prologue_)
        invocation.extend(invocation_)
    fn_decl = ", ".join(fn_decl)
    fn_ptr_decl = ", ".join(fn_ptr_decl)
    prologue = "" if len(prologue) == 0 else ("\n".join(prologue) + "\n")
    invocation = ", ".join(invocation)

    # return type
    return_type = "void"
    if len(e_name) == 3:
        arg_type, emit_thunk = convert_type(e_name[2])
        return_type = emit_thunk(None, arg_type, True)

    # Substitute the informations into the template
    # Note: The loader template should define the macro
    #    BASM_LOADER_IMAGEBASE, which loads the binary only once
    #    and returns the in-memory imagebase of the binary.
    #    It is best implemented as a function call, like `get_imagebase()`.
    template = "\n".join([
        "#if defined(BASM_LOADER_IMAGEBASE)",
        "{5} {0}({1}) {{",
        "    {3}return (({5} (BASMCALL *)({2}))(BASM_LOADER_IMAGEBASE + {6}))({4});",
        "}}",
        "#endif"
    ])
    return template.format(method_name, fn_decl, fn_ptr_decl, prologue, invocation, return_type, e_offset)