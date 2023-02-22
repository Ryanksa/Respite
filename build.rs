fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("proto/api.proto")?;
    tonic_build::compile_protos("proto/auth.proto")?;
    tonic_build::compile_protos("proto/store.proto")?;
    tonic_build::compile_protos("proto/menu.proto")?;
    tonic_build::compile_protos("proto/waiter.proto")?;
    Ok(())
}
