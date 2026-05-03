fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"], // Lokasi file proto
            &["proto"],                // Direktori include
        )?;
    Ok(())
}