fn main() {
    tonic_build::compile_protos("../proto/src/gas.proto").unwrap();
}
