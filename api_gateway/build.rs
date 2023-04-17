fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/user_management.proto"], &["proto/"])?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/account_service.proto"], &["proto/"])?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/deposit_service.proto"], &["proto/"])?;

    tonic_build::configure()
        .build_server(false)
        .build_client(true)
        .compile(&["proto/withdrawal_service.proto"], &["proto/"])?;

    Ok(())
}
