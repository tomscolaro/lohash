

use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::hash::compare_hash;

#[derive(Debug)]
pub struct VecTree {
    pub upper_threshold: f32,
    pub lower_threshold: f32,
    pub num_hashes: usize,
    pub random_noise_multi: Vec<u64>,
    pub random_noise_add: Vec<u64>,
}

#[derive(Debug)]
pub struct VecNode {
    pub id: i32,
    pub idx: i32,
    pub data: String,
    pub data_name: String,
    pub hash: Vec<f32>,
    pub hash_comp_value: Option<f32>
}

#[derive(Debug,  serde::Deserialize, serde::Serialize)]
pub struct  VecOutputStore {
    pub id: i32,
    pub idx: i32,
    pub data: String,
    pub data_name: String,
    pub hash_comp_value: Option<f32>,
    pub group: Option<i32>
}


impl VecTree {
    pub fn new(num_hashes:usize, upper_threshold:f32, lower_threshold:f32) -> Self {
            let mut rng = StdRng::seed_from_u64(72);

            //BOTH OF THESE RANDOM NOISE VECTORS TO BE BORROWED LATER
            let random_noise_multi: Vec<u64> = (0..num_hashes).map(|_|rng.gen_range(0..100)).collect(); 
            let random_noise_add: Vec<u64> = (0..num_hashes).map(|_| rng.gen_range(0..100)).collect();  
        
            VecTree{
  
                upper_threshold,
                lower_threshold,
                num_hashes,
  
                random_noise_multi,
                random_noise_add,
            }

    }

    pub fn place_node(&mut self,  input: &mut VecNode, vector: &mut Vec<Vec<VecNode>>) -> (f32, f32) {
    
        for (i, v) in vector.iter().enumerate(){

            let comp =  compare_hash(v[0].get_hash(), input.get_hash(), self.num_hashes);

            if comp >= self.upper_threshold {
                //add input to current vec
                return (i as f32, comp); 

            } 
            
            if comp >= self.lower_threshold  {
                //check against ny of the current vec 

                for (i, _second_vec) in v.iter().enumerate(){

                    let second_search =  compare_hash(v[i].get_hash(), input.get_hash(), self.num_hashes);

                    if second_search >= self.upper_threshold {
                        return (i as f32, comp)
                    }

                }    

            }
      
        }

        (-1.0, 1.0)
    }

    pub fn process_output(&self, input_vecs: &mut Vec<Vec<VecNode>>) -> Vec<VecOutputStore> { 
        let mut results:Vec<VecOutputStore> = vec![];
        

        for (i, v) in input_vecs.iter().enumerate() {
            for (j, _second_vec) in v.iter().enumerate(){
                let res = input_vecs[i][j].get_data(i as i32);
                results.push(res)
           
            }    

        }
      
        results
       
    }


}

impl VecNode {

    pub fn get_data(&self, output_iter: i32) -> VecOutputStore {
        VecOutputStore{
            id: self.id,
            idx: self.idx,
            data: self.data.clone(),
            data_name: self.data_name.clone(),
            hash_comp_value: self.hash_comp_value,
            group: Some(output_iter),
         }
    }


    pub fn get_hash(&self) -> Vec<f32> {
        self.hash.clone()
    }




}