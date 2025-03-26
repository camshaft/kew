use std::{fs, io::Write, path::Path};
use xshell::{cmd, Shell};

fn main() {
    let sh = Shell::new().unwrap();
    build(&sh, true);
}

fn build(sh: &Shell, opt: bool) {
    // if cmd!(sh, "which wasm-bindgen").quiet().run().is_err() {
    //     cmd!(sh, "cargo install wasm-bindgen-cli@0.2.100")
    //         .run()
    //         .unwrap();
    // }

    let _ = opt;

    cmd!(
        sh,
        "cargo build --target wasm32-unknown-unknown --release --workspace --exclude xtask"
    )
    .run()
    .unwrap();

    sh.create_dir("target/wasm-opt").unwrap();

    let script_time = fs::metadata(file!()).unwrap().modified().unwrap();

    for file in fs::read_dir("target/wasm32-unknown-unknown/release").unwrap() {
        let file = file.unwrap();
        let path = file.path();

        if path.extension().is_none_or(|ext| ext != "wasm") {
            continue;
        }

        let name = path.file_stem().unwrap().to_str().unwrap();
        let src_time = path.metadata().unwrap().modified().unwrap();
        let target_dir = Path::new("src/sims").join(name);
        let target_file = target_dir.join("build.js");

        if let Some(target_time) = target_file.metadata().ok().and_then(|m| m.modified().ok()) {
            if target_time > src_time && target_time > script_time {
                continue;
            }
        };

        let opt = Path::new("target/wasm-opt")
            .join(name)
            .with_extension("wasm");

        cmd!(sh, "wasm-opt --output {opt} -Os {path}")
            .run()
            .unwrap();

        cmd!(
            sh,
            "wasm-bindgen {opt} --out-dir {target_dir} --out-name build"
        )
        .run()
        .unwrap();

        let v = {
            let _dir = sh.push_dir(&target_dir);
            emit_interface(sh, name)
        };

        sh.write_file(target_dir.with_extension("ts"), v).unwrap();
    }
}

fn emit_interface(sh: &Shell, name: &str) -> Vec<u8> {
    let types = sh.read_file("build.d.ts").unwrap();

    let mut classes = vec![];

    for line in types.lines() {
        if let Some(name) = line.strip_prefix("export class ") {
            let name = name.strip_suffix(" {").unwrap();
            classes.push(Cls {
                name: name.to_string(),
                properties: vec![],
            });
            continue;
        }

        let Some(current) = classes.last_mut() else {
            continue;
        };

        let Some((prop, ty)) = line.split_once(": ") else {
            continue;
        };

        let prop = prop.trim();

        if prop.contains('(') {
            continue;
        }

        let ty = ty.trim_end_matches(';');

        current.properties.push(Property {
            name: prop.to_string(),
            ty: ty.to_string(),
        });
    }

    let mut out = vec![];
    let mut indent = "";

    macro_rules! w {
        () => {
            writeln!(out, "{indent}").unwrap()
        };
        ($($tt:tt)*) => {
            writeln!(out, "{indent}{}", format_args!($($tt)*)).unwrap()
        };
    }
    macro_rules! wl {
        ($v:expr) => {
            w!("{}", $v)
        };
    }

    let entry = format!("./{name}/build.js");
    wl!("// @ts-nocheck");
    w!("import * as wasm from {entry:?};");
    wl!("import { Sim as __Sim } from '~/data/sim.ts';");

    let use_memo = "_useMemo";
    let use_effect = "_useEffect";
    let use_state = "_useState";
    let use_ref = "_useRef";
    let throttle = "throttle";

    w!(
        "import {{ useMemo as {use_memo}, useEffect as {use_effect}, useState as {use_state}, useRef as {use_ref} }} from 'react';"
    );
    w!("import {{ throttle as {throttle} }} from 'throttle-debounce';");
    w!();

    wl!("export type SimTransform<T> = (result: __Sim | null) => T;");

    for cls in classes {
        let Cls { name, properties } = cls;

        w!("export interface {name}Props {{");
        for Property { name, ty } in &properties {
            w!("  {name}?: {ty};");
        }
        wl!("}");
        w!();

        w!("export function use{name}<T>(props: {name}Props, transform: SimTransform<T>): T {{");
        indent = "  ";

        {
            w!("let [_output, _set_output] = {use_state}(null);");

            wl!("if (_output == null) _output = transform(_output)");

            w!("const transformCb = {use_ref}(transform);");

            w!("const _instance = {use_memo}(() => {{");
            w!("  const i = new wasm.{name}() as {name};");
            wl!("  let _gen = 0;");
            wl!("  const _run = i._run;");
            w!("  i.run = {throttle}(500, run);");
            wl!("  function run(");
            for Property { name, ty } in &properties {
                w!("    {name}: {ty} | undefined, ");
            }
            wl!("  ) {{");
            for Property { name, .. } in &properties {
                w!("    if (typeof {name} !== 'undefined') this.{name} = {name};");
            }
            wl!("    const _ret = _run.call(this);");
            wl!("    const transformed = transformCb.current(_ret);");
            wl!("    _set_output(transformed);");
            wl!("  }}");
            w!("  return i;");
            wl!("}, []);");
            w!();

            // w!("{use_effect}(() => (");
            // wl!("  () => _instance.free()");
            // wl!("), []);");
            // w!();

            w!("{use_effect}(() => _instance.run(");
            for Property { name, .. } in &properties {
                w!("    props.{name},");
            }
            w!("), [");
            for Property { name, .. } in &properties {
                w!("  props.{name},");
            }
            wl!("]);");
            w!();

            w!("return _output;")
        }

        indent = "";
        wl!("}");
        w!();
    }

    out
}

struct Cls {
    name: String,
    properties: Vec<Property>,
}

struct Property {
    name: String,
    ty: String,
}
