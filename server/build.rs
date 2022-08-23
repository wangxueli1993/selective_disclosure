// pub fn main() -> Result<(), Box<dyn std::error::Error>> {
//     tonic_build::compile_protos("../protos/solution/scd/scd.proto")?;
//     Ok(())
// }
pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
    .out_dir("../protos/src/")
    .compile(&["../protos/solution/scd/scd.proto"],&["../protos"])?;
    Ok(())
}