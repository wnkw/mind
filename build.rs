fn main() {
    tonic_build::compile_protos("proto/logical_lambda.proto").unwrap();
}
