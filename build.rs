fn main() {
    protobuf_codegen::Codegen::new()
        .out_dir("src")
        .include("src")
        .input("src/protos/example.proto")
        .run_from_script();
}
