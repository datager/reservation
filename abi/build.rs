use std::fs;
use std::process::Command;

fn main() {
    tonic_build::configure()
        .out_dir("src/pb")
        .type_attribute(
            "reservation.ReservationStatus",
            "#[derive(sqlx::Type)]
        ",
        )
        .compile(&["protos/reservation.proto"], &["protos"])
        .unwrap();

    let _ = fs::remove_file("src/pb/google.protobuf.rs");

    Command::new("cargo").args(&["fmt"]).output().unwrap();

    println!("cargo:rerun-if-changed=protos/reservation.proto");
}