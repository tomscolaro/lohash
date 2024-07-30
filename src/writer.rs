use std::error::Error;

use csv::Writer;

use crate:: { //tree::OutputStore, 
    vectortree::VecOutputStore};

#[derive(Debug)]
pub struct CsvWriter {
    pub output_file_path: String
}



impl CsvWriter {
    pub fn new(output_file_path:String) -> Self {

        CsvWriter{
            output_file_path
        }
    }

    // pub fn write_output_file_from_csv(&self, res: Vec<Vec<OutputStore>>) -> Result<(), Box<dyn Error>> {
    //     let mut wtr = Writer::from_path(self.output_file_path.clone())?;
        
    //     for vec_group in res.iter() {
            
    //         for row in vec_group.iter() {

    //             let _ = wtr.serialize(row);
    //         }

    //     }

    //     Ok(())
    // }


    pub fn write_output_file_from_vector_nodes(&self, res: Vec<VecOutputStore>) -> Result<(), Box<dyn Error>> {
        let mut wtr = Writer::from_path(self.output_file_path.clone())?;
        
        for row in res.iter() {
    
            let _ = wtr.serialize(row);
        
        }

        Ok(())
    }


}