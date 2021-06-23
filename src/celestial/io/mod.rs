use std::fs::File;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Harmonic {
    i: usize,
    j: usize,
    ci: f64,
    cj: f64,
    di: f64,
    dj: f64,
}

struct EGM {
    path: String,
}

impl EGM {
    pub fn from_file(path: String) -> Self {
        Self {
            path,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use csv::ReaderBuilder;
    use nalgebra::{DMatrix, Vector6, RowVector6, Vector2};
    use ndarray::{Array3, Array1};
    use std::error::Error;
    // use std::fs::read;
    // use regex::Regex;

    const TEST_EGM96: &str = "/home/ggarrett/PycharmProjects/astroforge/astroforge-celestial/src/io/EGM96/EGM96.csv";
    // const TEST_RE: &str = r"\s+(?P<M>\w+)\s+(?P<N>[\w,.+-]+)\s+(?P<CI>[\w,.+-]+)\s+(?P<CJ>[\w,.+-]+)\s+(?P<ECI>[\w,.+-]+)\s+(?P<ECJ>[\w,.+-]+)";


    #[test]
    fn test_1() -> std::io::Result<()>{
        let file = File::open(TEST_EGM96)?;
        let reader = std::io::BufReader::new(file);
        // let mut reader = csv::Reader::from_reader(std::io::stdin());

        let mut rdr = ReaderBuilder::new()
            .from_reader(reader);

        use ndarray::array;
        use ndarray::s;

        use std::collections::HashMap;
        let mut harmonics_c_val: HashMap<(usize, usize), (f64, f64)> = HashMap::new();
        let mut harmonics_c_err: HashMap<(usize, usize), (f64, f64)> = HashMap::new();

        for (i, result) in rdr.deserialize().enumerate() {
            let record: Harmonic = result?;
            harmonics_c_val.insert((record.i, record.j), (record.ci, record.cj));
            harmonics_c_err.insert((record.i, record.j), (record.di, record.dj));

            // test.set_row(i, &RowVector6::new(
            //     record.i,
            //     record.j,
            //     record.ci,
            //     record.cj,
            //     record.di,
            //     record.dj,
            // ));
            // println!("{:?}", record);
            // println!("{:?}", result);
        }
        println!("{:?}", harmonics_c_val.get(&(30,30)));
        println!("{:?}", harmonics_c_err.get(&(30,30)));

        // if let Some(result) = rdr.records().next() {
        //     let record = result?;
        //     println!("{:?}", record);
        //     // assert_eq!(record, vec!["Boston", "United States", "4628910"]);
        //     // Ok(())
        // } else {
        //     // Err(From::from("expected at least one record but got none"))
        // }

        // let re = Regex::new(TEST_RE).unwrap();
        // for line in reader.lines() {
        //     match re {
        //         _ => {}
        //     }
        //     println!("{}", line?);
        // }

        // let mut contents = String::new();
        // file.read_to_string(&mut contents)?;
        // print!("{}", contents);
        // file.write_all(b"Hello, world!")?;
        Ok(())
    }
}