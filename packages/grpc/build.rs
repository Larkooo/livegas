fn main() {
    tonic_build::compile_protos("../proto/src/helloworld.proto").unwrap();
}
