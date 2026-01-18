fn main() {
    tonic_prost_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(&["proto/user.proto", "proto/org.proto"], &["proto"])
        .expect("Failed to compile proto files");
}
