use cc;
use glob::glob;
use std::path::Path;

fn main() {
    let c_api_src_dir = Path::new("c_api/src");
    let c_api_sources: Vec<_> = glob(c_api_src_dir.join("*.cpp").to_str().unwrap())
        .unwrap()
        .map(|x| x.unwrap())
        .collect();
    let core_src_dir = Path::new("submodules/zxing-cpp/core/src");
    let core_sources: Vec<_> = glob(core_src_dir.join("**/*.cpp").to_str().unwrap())
        .unwrap()
        .map(|x| x.unwrap())
        .collect();

    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .include(core_src_dir)
        .files(core_sources)
        .compile("zxing_core");
    cc::Build::new()
        .cpp(true)
        .flag("-std=c++14")
        .flag("-v")
        .flag("-g")
        .include(c_api_src_dir)
        .include(core_src_dir)
        .files(c_api_sources)
        .compile("zxing_c_api");
}
