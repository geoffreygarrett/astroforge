use std::fs::File;
// use std::io::prelude::*;

// fn main() -> std::io::Result<()> {
//     let mut file = File::create("foo.txt")?;
//     file.write_all(b"Hello, world!")?;
//     Ok(())
// }

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

#[cfg(test)]
mod test {
    use super::*;
    use csv::ReaderBuilder;
    // use std::fs::read;
    // use regex::Regex;

    const TEST_EGM96: &str = "/home/ggarrett/PycharmProjects/astroforge/astroforge-celestial/src/io/EGM96/EGM96.csv";
    // const TEST_RE: &str = r"\s+(?P<M>\w+)\s+(?P<N>[\w,.+-]+)\s+(?P<CI>[\w,.+-]+)\s+(?P<CJ>[\w,.+-]+)\s+(?P<ECI>[\w,.+-]+)\s+(?P<ECJ>[\w,.+-]+)";


    #[test]
    fn test_1() -> std::io::Result<()> {
        let file = File::open(TEST_EGM96)?;
        let reader = std::io::BufReader::new(file);
        // let mut reader = csv::Reader::from_reader(std::io::stdin());

        let mut rdr = ReaderBuilder::new()
            .from_reader(reader);

        for result in rdr.deserialize() {
            let record : Harmonic = result?;
            println!("{:?}", record);
            // println!("{:?}", result);
        }

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