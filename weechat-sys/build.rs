use bindgen::Bindings;
use std::env;
use std::path::PathBuf;

fn build(file: &str) -> Result<Bindings, ()> {
    const INCLUDED_TYPES: &[&str] = &[
        "t_weechat_plugin",
        "t_gui_buffer",
        "t_gui_nick",
        "t_gui_nick_group",
        "t_hook",
        "t_hdata",
    ];
    const INCLUDED_VARS: &[&str] = &[
        "WEECHAT_PLUGIN_API_VERSION",
        "WEECHAT_HASHTABLE_INTEGER",
        "WEECHAT_HASHTABLE_STRING",
        "WEECHAT_HASHTABLE_POINTER",
        "WEECHAT_HASHTABLE_BUFFER",
        "WEECHAT_HASHTABLE_TIME",
        "WEECHAT_HOOK_SIGNAL_STRING",
        "WEECHAT_HOOK_SIGNAL_INT",
        "WEECHAT_HOOK_SIGNAL_POINTER",
    ];
    let mut builder = bindgen::Builder::default().rustfmt_bindings(true);

    builder = builder.header(file);

    for t in INCLUDED_TYPES {
        builder = builder.whitelist_type(t);
    }

    for v in INCLUDED_VARS {
        builder = builder.whitelist_var(v);
    }

    builder.generate()
}

fn main() {
    let bindings = build("src/wrapper.h");

    let bindings = match bindings {
        Ok(b) => b,
        Err(_) => build("src/weechat-plugin.h").expect("Unable to generate bindings"),
    };

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
