use rand::{rngs::StdRng, Rng, SeedableRng};
use crate::hash::{compare_hash, compute_hash};

#[derive(Debug)]
pub struct Tree {
    pub node: Option<Box<TreeNode>>,
    pub upper_threshold: f32,
    pub lower_threshold: f32,
    pub num_hashes: usize,
    pub num_buckets: usize,
    pub random_noise_multi: Vec<u64>,
    pub random_noise_add: Vec<u64>,
    pub group_iter_counter: i32
}

#[derive(Debug)]
pub struct TreeNode {
    pub id: i32,
    pub idx: i32,
    pub data: String,
    pub data_name: String,
    pub tokens: Vec<String>,
    pub hash_comp_value: Option<f32>,
    pub is_parent: bool,
    pub left_hash_node: Option<Box<TreeNode>>,
    pub right_hash_node: Option<Box<TreeNode>>

}
#[derive(Debug,  serde::Deserialize, serde::Serialize)]
pub struct  OutputStore {
    pub id: i32,
    pub idx: i32,
    pub data: String,
    pub data_name: String,
    pub hash_comp_value: Option<f32>,
    pub group: Option<i32>
}


impl Tree {
    pub fn new(num_hashes:usize, num_buckets:usize, upper_threshold:f32, lower_threshold:f32) -> Self {
            let mut rng = StdRng::seed_from_u64(72);

            //BOTH OF THESE RANDOM NOISE VECTORS TO BE BORROWED LATER
            let random_noise_multi: Vec<u64> = (0..num_hashes).map(|_|rng.gen_range(0..100)).collect(); 
            let random_noise_add: Vec<u64> = (0..num_hashes).map(|_| rng.gen_range(0..100)).collect();  
        
            Tree{
                node:None,
                upper_threshold,
                lower_threshold,
                num_hashes,
                num_buckets,
                random_noise_multi,
                random_noise_add,
                group_iter_counter: 0
            }

    }

    pub fn place_node(&mut self, input:Box<TreeNode>){
        match self.node {    
            Some(ref mut node) => 
                {
                node.descend_to_children(input, 
                              self.upper_threshold, self.lower_threshold, 
                                          self.num_hashes, self.num_buckets, 
                                          &self.random_noise_multi, &self.random_noise_add);
                 
                node.make_parent()
            
                },
            None => self.node = std::option::Option::Some(input),
        }
    }

    pub fn print_tree(&mut self){
        println!("######### Grouping Tree #################");
        print!("{}",self.node.as_mut().unwrap().print_tree()+ {"\n"});

    }


    pub fn proccess_output(&mut self) -> Vec<Vec<OutputStore>>{
        let mut r = vec![];
        self.group_iter_counter = 0;

        match self.node {    
            Some(ref mut node) => 
                {
                node.process_output(&mut r, self.group_iter_counter);
            
                },
            None => panic!()
        }

        r

    }   



}

impl TreeNode {
    pub fn descend_to_children(&mut self, 
                                input:Box<TreeNode>, 
                                upper_thres:f32, lower_thres:f32, 
                                num_hashes:usize, num_buckets:usize, 
                                random_noise_multi:&Vec<u64>, random_noise_add:&Vec<u64>){

        let self_hash = compute_hash(&self.tokens, 
                                                num_hashes, num_buckets, 
                                                random_noise_multi, random_noise_add);
        
        let input_hash = compute_hash(&input.tokens, 
                                                num_hashes, num_buckets, 
                                                random_noise_multi, random_noise_add);
        
        let comp_val = compare_hash(self_hash, input_hash, num_hashes);

 
        let below_min_thres = comp_val <= lower_thres;
        let above_threshold = comp_val >= upper_thres;
        
        if below_min_thres {
            self.move_child_right(input, upper_thres, lower_thres, num_hashes, num_buckets, random_noise_multi, random_noise_add)


        } else if above_threshold {
            // if left node is null, we can automatically add it to the end of the left list
            self.accelerate_to_child(input, comp_val);
                  
        } else { //must be between between thresholds
            
             //we can't check the left path, so move it to the right            
            match self.left_hash_node {
                Some(ref mut _left_hash_node) => 
                {
                    // println!("{}", "We are in the else statment");
                    if self.check_left_path(&input, upper_thres, num_hashes, num_buckets,random_noise_multi,random_noise_add) { 
                        self.accelerate_to_child(input, comp_val)

                    } else { // we've rejected the left path       
                        self.move_child_right(input, 
                                            upper_thres, lower_thres, 
                                            num_hashes, num_buckets, 
                                            random_noise_multi, random_noise_add)
                    
                    }

                },
                None => 
                {
                    match self.right_hash_node {
                        Some(ref mut right_hash_node) => 
                            right_hash_node.descend_to_children(input, 
                                                                upper_thres, lower_thres, 
                                                                num_hashes, num_buckets, 
                                                                random_noise_multi, random_noise_add),

                        None =>
                        {                 
                            self.right_hash_node = Some(input);
                            match self.right_hash_node {
                                Some(ref mut right_hash_node) => right_hash_node.make_parent(),
                                None => panic!(),
                            }       
                        }
                    }            
                }        
            }
        }
    }

    pub fn move_child_right(&mut self, 
                            input:Box<TreeNode>, 
                            upper_thres:f32, lower_thres:f32, 
                            num_hashes:usize, num_buckets:usize, 
                            random_noise_multi:&Vec<u64>, random_noise_add:&Vec<u64> ){
        
    
        match self.right_hash_node  {
            Some(ref mut _right_hash_node) => 
            {
                match self.right_hash_node {
                    Some(ref mut right_hash_node) => right_hash_node.descend_to_children(input, 
                        upper_thres, lower_thres, 
                        num_hashes, num_buckets, 
                        random_noise_multi, random_noise_add),
                     None => panic!(),
    
                }
            },

            None => {
                self.right_hash_node = Some(input);
                // self.right_hash_node.as_mut().unwrap().make_parent();
                match self.right_hash_node {
                    Some(ref mut right_hash_node) => right_hash_node.make_parent(),
                    None => panic!(),
                }
            }
        }
   
    }
        
    pub fn accelerate_to_child(&mut self, input:Box<TreeNode>, comp_value:f32){
            match self.left_hash_node {
                Some(ref mut left_hash_node) => 
                    left_hash_node.accelerate_to_child(input, comp_value),
              
                None => {
                    self. left_hash_node = Some(input);
                    match self.left_hash_node {
                        Some(ref mut left_hash_node) => 
                            left_hash_node.set_comp_value(comp_value),
                        
                        None => panic!()
                    }
                }

            }       
    }
    
    pub fn check_left_path(&mut self, 
                            input:&Box<TreeNode>, 
                            upper_thres:f32,     
                            num_hashes:usize, num_buckets:usize, 
                            random_noise_multi:&Vec<u64>, random_noise_add:&Vec<u64>  ) -> bool {
        
        let self_hash = compute_hash(&self.tokens, 
                                                num_hashes, num_buckets, 
                                random_noise_multi, random_noise_add);

        let input_hash = compute_hash(&input.tokens, 
                                                    num_hashes, num_buckets, 
                                    random_noise_multi, random_noise_add);

        let comp_val = compare_hash(self_hash, input_hash, num_hashes);

        if comp_val >= upper_thres{
            true;
        } 
        
        match self.left_hash_node {
                Some(ref mut left_hash_node) =>   
                    return left_hash_node.check_left_path(&input, upper_thres, 
                                                            num_hashes, num_buckets,
                                                            random_noise_multi,random_noise_add),
                
                None => 
                    return false
        }
        
    }


    pub fn make_parent(&mut self){
        self.is_parent = true;
        self.hash_comp_value = Some(1.0);
    }

    pub fn set_comp_value(&mut self, comp_val:f32){
        self.hash_comp_value = Some(comp_val);
    }

    pub fn print_tree(&mut self) -> String{
        let mut s =  self.id.to_string()+ ", idx:" + &self.idx.to_string() + "     = " + &self.hash_comp_value.unwrap().to_string();

        if !self.left_hash_node.is_none() {
            s = s + {"\n-----"} + &self.left_hash_node.as_mut().unwrap().print_tree();
        }

        if !self.right_hash_node.is_none() {
            s = s + {"\n"} +  &self.right_hash_node.as_mut().unwrap().print_tree();
        }

        s
    }
    
    pub fn process_output(&self, res: &mut Vec<Vec<OutputStore>>, output_iter: i32){ 
        let mut results:Vec<OutputStore> = vec![];

        if self.is_parent {
                match &self.left_hash_node {
                    Some(left_hash_node) =>   { //return parent and left children                        
                        results.push(self.get_data(output_iter)); 
                        left_hash_node.process_left_children(&mut results, output_iter)}
                        ,
                    None =>  results.push(self.get_data(output_iter)) //only return parent
                }
        }

        if !self.right_hash_node.is_none() { // continue down right side of tree
            match &self.right_hash_node {
                Some(right_hash_node) =>   {
                    right_hash_node.process_output(res, output_iter+1)
                }, 
                None => return     
            }
        }


        res.push(results)
    }


    pub fn process_left_children(&self, res: &mut Vec<OutputStore>, output_iter: i32){
        // println!("in left {}", self.idx);
        res.push(self.get_data(output_iter));

        match &self.left_hash_node {
            Some(left_hash_node) =>   
                left_hash_node.process_left_children(res, output_iter),
            
            None => return
        }


    }



    pub fn get_data(&self, output_iter: i32) -> OutputStore {
        OutputStore{
            id: self.id,
            idx: self.idx,
            data: self.data.clone(),
            data_name: self.data_name.clone(),
            hash_comp_value: self.hash_comp_value,
            group: Some(output_iter),
         }
    }


}