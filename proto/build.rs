pub fn main() {
    let proto_file = "StarRail.proto";
    if std::path::Path::new(proto_file).exists() {
        println!("cargo:rerun-if-changed={proto_file}");

        prost_build::Config::new()
            .out_dir("out/")
            .enum_attribute(".", "#[derive(EnumString)]")
            .compile_protos(&[proto_file], &["."])
            .unwrap();
    }
}
