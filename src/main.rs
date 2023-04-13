use nifti::{IntoNdArray, NiftiObject, NiftiVolume};
use serde::Deserialize;
use std::io::Write;

fn main() {
    let args = std::env::args().collect::<Vec<_>>();
    if args.len() < 4 {
        eprintln!("ERROR: Missing command line argument(s)");
        println!(
            "Usage: {program_name} INPUTFILE OPTIONSFILE OUTPUTFILE",
            program_name = args
                .first()
                .unwrap_or(&String::from("dicon-nifti-formatting.exe"))
        );
        std::process::exit(1);
    }

    let input_file_path = args.get(1).unwrap();
    let options_file_path = args.get(2).unwrap();
    let output_file_path = args.get(3).unwrap();
    if let Err(err) = convert(input_file_path, options_file_path, output_file_path) {
        eprintln!("ERROR: {err}");
        std::process::exit(1);
    }
}

fn convert(
    input_file_path: &str,
    options_file_path: &str,
    output_file_path: &str,
) -> Result<(), String> {
    let nifti_object = nifti::ReaderOptions::new()
        .read_file(input_file_path)
        .map_err(|err| err.to_string())?;
    let slices = nifti_object
        .volume()
        .into_ndarray::<f64>()
        .map_err(|err| err.to_string())?;
    write(
        nifti_object,
        slices,
        input_file_path,
        options_file_path,
        output_file_path,
    )
    .map_err(|err| err.to_string())?;
    Ok(())
}

#[derive(Deserialize)]
struct Options {
    #[serde(rename(deserialize = "correct"))]
    correct: i32,
    #[serde(rename(deserialize = "ms"))]
    ms: i32,
    #[serde(rename(deserialize = "lower Hz"))]
    lower_hz: f32,
    #[serde(rename(deserialize = "upper Hz"))]
    upper_hz: f32,
}

fn write(
    nifti_object: nifti::object::GenericNiftiObject<nifti::InMemNiftiVolume>,
    slices: ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<ndarray::IxDynImpl>>,
    input_file_path: &str,
    options_file_path: &str,
    output_file_path: &str,
) -> Result<(), std::io::Error> {
    let options_file_reader = std::io::BufReader::new(std::fs::File::open(options_file_path)?);
    let options: Options = serde_json::from_reader(options_file_reader)?;
    let mut output_file_writer = std::io::BufWriter::new(std::fs::File::create(output_file_path)?);
    writeln!(
        output_file_writer,
        "BESA_SA_IMAGE:2.0

Data file:          {input_file_path}
Condition:          Correct  Notch filter: 60 Hz
Correct {correct} : {ms} ms {lower_hz:.1}-{upper_hz:.1} Hz MSBF (TF) q%

Grid dimensions ([min] [max] [nr of locations]):
X: {x_min:.6} 70.000000 {x_dim}
Y: {y_min:.6} 71.629997 {y_dim}
Z: {z_min:.6} 77.379997 {z_dim}",
        correct = options.correct,
        ms = options.ms,
        lower_hz = options.lower_hz,
        upper_hz = options.upper_hz,
        x_min = nifti_object.header().srow_x[3],
        x_dim = nifti_object.volume().dim()[0],
        y_min = nifti_object.header().srow_y[3],
        y_dim = nifti_object.volume().dim()[1],
        z_min = nifti_object.header().srow_z[3],
        z_dim = nifti_object.volume().dim()[2],
    )?;
    write!(output_file_writer,
    "=============================================================================================="
    )?;
    for (z_index, slice) in slices.axis_iter(ndarray::Axis(2)).enumerate() {
        writeln!(output_file_writer)?;
        writeln!(output_file_writer, "Z: {}", z_index)?;
        for column in slice.columns() {
            if let Some(first) = column.first() {
                write!(output_file_writer, "{:.10}", first)?;
            }
            for sample in column.iter().skip(1) {
                write!(output_file_writer, " {:.10}", sample)?;
            }
            writeln!(output_file_writer)?;
        }
    }
    Ok(())
}
