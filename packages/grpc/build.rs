fn main() {
    tonic_build::compile_protos("../proto/gas.proto").unwrap();
}
