use crate::{
    args::{self, DislodgeParams},
    etcher,
};

pub async fn run_dislodge(args: DislodgeParams) -> anyhow::Result<()> {
    let out_data = etcher::new(
        &args
            .in_path
            .expect("Input path was not provided by the user"),
        1,
    )?;

    etcher::write_bytes(
        &args
            .out_path
            .expect("Output path was not provided by the user"),
        out_data,
    )?;

    println!(
        "Dislodged data written to {}",
        args.out_path
            .expect("Output path was not provided by the user")
    );
    Ok(())
}
