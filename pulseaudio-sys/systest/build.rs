extern crate ctest;

fn main() {
    let mut cfg = ctest::TestGenerator::new();
    cfg.header("pulse/pulseaudio.h");

    cfg.field_name(|_, f| match f {
        "kind" => "type".to_string(),
        _ => f.to_string(),
    });

    cfg.skip_signededness(|s| match s {
        s if s.ends_with("_cb_t") => true,
        "pa_poll_func" => true,
        _ => false,
    });

    cfg.generate("../src/lib.rs", "all.rs");
}
