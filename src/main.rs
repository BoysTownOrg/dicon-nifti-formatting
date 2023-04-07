use nifti::{IntoNdArray, NiftiObject, NiftiVolume, ReaderOptions};
use std::io::Write;

fn main() -> Result<(), std::io::Error> {
    let nifti = ReaderOptions::new()
        .read_file("a.nii")
        .expect("Should be able to read nifti file.");
    let slices = nifti
        .volume()
        .into_ndarray::<f32>()
        .expect("Should be able to convert volume into ndarray.");
    let mut file = std::io::BufWriter::new(
        std::fs::File::create("out.dat").expect("Should be able to create a file."),
    );
    writeln!(file,
"=============================================================================================="
    )?;
    for (z_index, slice) in slices.axis_iter(ndarray::Axis(2)).enumerate() {
        writeln!(file, "Z: {}", z_index)?;
        for row in slice.columns() {
            if let Some(first) = row.first() {
                write!(file, "{:.10}", first)?;
            }
            for sample in row.iter().skip(1) {
                write!(file, " {:.10}", sample)?;
            }
            writeln!(file)?;
        }
        writeln!(file)?;
    }
    Ok(())
}
