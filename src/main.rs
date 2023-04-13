use nifti::{IntoNdArray, NiftiObject, NiftiVolume};
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 3 {
        eprintln!("ERROR: Missing command line argument(s)");
        println!(
            "Usage: {program_name} INPUTFILE OUTPUTFILE",
            program_name = args
                .first()
                .unwrap_or(&String::from("dicon-nifti-formatting.exe"))
        );
        std::process::exit(1);
    }

    let input_file_path = args.get(1).unwrap();
    let output_file_path = args.get(2).unwrap();
    if let Err(err) = convert(input_file_path, output_file_path) {
        eprintln!("ERROR: {err}");
        std::process::exit(1);
    }
}

fn convert(input_file_path: &str, output_file_path: &str) -> Result<(), String> {
    let nifti_object = nifti::ReaderOptions::new()
        .read_file(input_file_path)
        .map_err(|err| err.to_string())?;
    let slices = nifti_object
        .volume()
        .into_ndarray::<f64>()
        .map_err(|err| err.to_string())?;
    write(nifti_object, slices, input_file_path, output_file_path)
        .map_err(|err| err.to_string())?;
    Ok(())
}

fn write(
    nifti_object: nifti::object::GenericNiftiObject<nifti::InMemNiftiVolume>,
    slices: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<ndarray::IxDynImpl>>,
    input_file_path: &str,
    output_file_path: &str,
) -> Result<(), std::io::Error> {
    let mut file = std::io::BufWriter::new(std::fs::File::create(output_file_path)?);
    writeln!(
        file,
        "BESA_SA_IMAGE:2.0

Data file:          {input_file_path}
Condition:          Correct  Notch filter: 60 Hz
Correct 350 : 650 ms 8.0-14.0 Hz MSBF (TF) q%

Grid dimensions ([min] [max] [nr of locations]):
X: {x_min:.6} 70.000000 {x_dim}
Y: {y_min:.6} 71.629997 {y_dim}
Z: {z_min:.6} 77.379997 {z_dim}",
        x_min = nifti_object.header().srow_x[3],
        x_dim = nifti_object.volume().dim()[0],
        y_min = nifti_object.header().srow_y[3],
        y_dim = nifti_object.volume().dim()[1],
        z_min = nifti_object.header().srow_z[3],
        z_dim = nifti_object.volume().dim()[2],
    )?;
    write!(file,
    "=============================================================================================="
    )?;
    for (z_index, slice) in slices.axis_iter(ndarray::Axis(2)).enumerate() {
        writeln!(file)?;
        writeln!(file, "Z: {}", z_index)?;
        for column in slice.columns() {
            if let Some(first) = column.first() {
                write!(file, "{:.10}", first)?;
            }
            for sample in column.iter().skip(1) {
                write!(file, " {:.10}", sample)?;
            }
            writeln!(file)?;
        }
    }
    Ok(())
}
