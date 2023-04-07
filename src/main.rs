use nifti::{IntoNdArray, NiftiObject, NiftiVolume};
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let input_file_path = "in.nii";
    let nifti = nifti::ReaderOptions::new()
        .read_file(input_file_path)
        .expect("Should be able to read nifti file.");
    let slices = nifti
        .volume()
        .into_ndarray::<f64>()
        .expect("Should be able to convert volume into ndarray.");
    println!("header: {:?}", nifti.header());
    println!("slice order: {:?}", nifti.header().slice_order());
    println!("extensions: {:?}", nifti.extensions());
    println!("dimensions: {:?}", nifti.volume().dim());
    println!("data type: {:?}", nifti.volume().data_type());
    let mut file = std::io::BufWriter::new(std::fs::File::create("out.dat")?);
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
        x_min = nifti.header().srow_x[3],
        x_dim = nifti.volume().dim()[0],
        y_min = nifti.header().srow_y[3],
        y_dim = nifti.volume().dim()[1],
        z_min = nifti.header().srow_z[3],
        z_dim = nifti.volume().dim()[2],
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
