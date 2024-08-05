use std::process::Command;

fn main() {
    let mut build_date = build_data::format_date(build_data::now());

    if build_date.ends_with('Z') {
        build_date.pop();
    }

    // Run `npm install` to install Tailwind CSS
    let status = Command::new("npm")
        .args(["install"])
        .status()
        .expect("failed to install node dependencies");
    if !status.success() {
        panic!("failed to install node dependencies");
    }

    // Run `npm run build` to build the Tailwind CSS
    let status = Command::new("npm")
        .args(["run", "build"])
        .status()
        .expect("failed to build Tailwind CSS");
    if !status.success() {
        panic!("failed to build Tailwind CSS");
    }

    println!("cargo:rerun-if-changed=templates");
    println!("cargo:rustc-env=CARGO_BUILD_DATE={}", build_date);

    build_data::no_debug_rebuilds();
}
