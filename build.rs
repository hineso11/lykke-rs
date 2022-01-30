use std::error::Error;
use tonic_build::compile_protos;

fn main() -> std::io::Result<()> {
    let mut proto_file_paths = vec![
        "common.proto",
        "isalive.proto",
        "privateService.proto",
        "publicService.proto",
    ];

    let proto_file_paths: Vec<String> = proto_file_paths
        .into_iter()
        .map(|x| String::from(x))
        .collect();

    let file_path_root = format!("{}/libs/lykke-trading-api/grpc_proto_contracts/", env!("CARGO_MANIFEST_DIR"));
    let proto_path_arg = format!("--proto_path={}", file_path_root);

    tonic_build::configure()
        .protoc_arg(proto_path_arg)
        .build_client(true)
        .compile(&proto_file_paths, &[])
}

