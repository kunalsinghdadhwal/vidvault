use crate::{
    args::DislodgeParams,
    ethcer,
};

pub async fn run_dislodge(args: DislodgeParams) -> anyhow::Result<()> {
    let out_data = ethcer::read(
        &args
            .in_path
            .expect("Input path was not provided by the user"),
        1,
    )?;

    ethcer::write_bytes(
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
