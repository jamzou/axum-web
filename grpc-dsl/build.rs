fn main() {
    tonic_build::configure()
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile(&["proto/user.proto"], &["proto"])
        .unwrap();
}
