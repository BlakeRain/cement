use std::process::Command;

fn main() {
    let mut build_date = build_data::format_date(build_data::now()).expect("to get build date");

    if build_date.ends_with('Z') {
        build_date.pop();
    }

    let npm_commands = vec![
        vec!["install"],
        vec!["run", "build-css"],
        vec!["run", "build-highlight"],
        vec!["run", "copy-highlight"],
    ];

    for npm_command in &npm_commands {
        let status = Command::new("npm")
            .args(npm_command)
            .status()
            .unwrap_or_else(|_| panic!("failed to run npm command: {:?}", npm_command));

        if !status.success() {
            panic!("failed to run npm command: {:?}", npm_command);
        }
    }

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rerun-if-changed=style");
    println!("cargo:rustc-env=CARGO_BUILD_DATE={}", build_date);

    build_data::no_debug_rebuilds().expect("to disable debug rebuilds");
}
