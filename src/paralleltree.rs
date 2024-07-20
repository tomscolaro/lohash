// use futures::Future;
// use rand::{rngs::StdRng, Rng, SeedableRng};
// use crate::hash::{compare_hash, compute_hash};
// use crate::writer::CsvWriter;
// use tokio::sync::RwLock;


// #[derive(Debug)]
// pub struct ParallelTree {
//     pub node: Option<Arc<RwLock<ParallelTreeNode>>>,
//     pub upper_threshold: f32,
//     pub lower_threshold: f32,
//     pub num_hashes: usize,
//     pub num_buckets: usize,
//     pub random_noise_multi: Vec<u64>,
//     pub random_noise_add: Vec<u64>,
// }

// #[derive(Debug)]
// pub struct ParallelTreeNode {
//     pub id: i32,
//     pub tokens: Vec<String>,
//     pub hash_comp_value: Option<f32>,
//     pub is_parent: bool,
//     pub left_hash_node: Option<Arc<RwLock<ParallelTreeNode>>>,
//     pub right_hash_node: Option<Arc<RwLock<ParallelTreeNode>>>

// }

// impl ParallelTree {
//     pub async fn new(num_hashes:usize, num_buckets:usize, upper_threshold:f32, lower_threshold:f32) -> Self {
//             let mut rng = StdRng::seed_from_u64(72);

//             //BOTH OF THESE RANDOM NOISE VECTORS TO BE BORROWED LATER
//             let random_noise_multi: Vec<u64> = (0..num_hashes).map(|_|rng.gen_range(0..100)).collect(); 
//             let random_noise_add: Vec<u64> = (0..num_hashes).map(|_| rng.gen_range(0..100)).collect();  
        
//             ParallelTree{
//                 node:None,
//                 upper_threshold,
//                 lower_threshold,
//                 num_hashes,
//                 num_buckets,
//                 random_noise_multi,
//                 random_noise_add
//             }

//     }

//     pub async  fn place_node(&mut self, input: Arc<RwLock<ParallelTreeNode>> ){
//         match &self.node {    
//             Some(node) => 
//                 {
//                 let node_clone = Arc::clone(node);
//                 let mut node_guard = node_clone.write().await;
                
//                 node_guard.descend_to_children(input, 
//                               self.upper_threshold, self.lower_threshold, 
//                                           self.num_hashes, self.num_buckets, 
//                                           &self.random_noise_multi, &self.random_noise_add);
                 
//                 node_guard.make_parent();
            
//                 },
//             None => self.node = std::option::Option::Some(input),
//         }
//     }

//     pub fn print_tree(&mut self){
//         if let Some(node) = &self.node {
//             let node_guard = node.read();
//             println!("######### Grouping Tree #################");
//             // print!("{}", node_guard.print_tree().aw + {"\n"});

//         }
//     }


//     pub async fn proccess_output(&mut self, writer: &CsvWriter){

//     }



// }

// impl ParallelTreeNode {
//     pub async fn descend_to_children(&mut self, 
//                                 input:Arc<RwLock<ParallelTreeNode>>, 
//                                 upper_thres:f32, lower_thres:f32, 
//                                 num_hashes:usize, num_buckets:usize, 
//                                 random_noise_multi:&Vec<u64>, random_noise_add:&Vec<u64>){



//         let input_clone =Arc::clone(&input);
//         let mut input_guard = input_clone.read().await;
                
        

//         let self_hash = compute_hash(&self.tokens, 
//                                                 num_hashes, num_buckets, 
//                                                 random_noise_multi, random_noise_add);
        
//         let input_hash = compute_hash(&input_guard.tokens, 
//                                                 num_hashes, num_buckets, 
//                                                 random_noise_multi, random_noise_add);
        
//         let comp_val = compare_hash(self_hash, input_hash, num_hashes);

 
//         let below_min_thres = comp_val <= lower_thres;
//         let above_threshold = comp_val >= upper_thres;
        
//         if below_min_thres {
//             self.move_child_right(input, upper_thres, lower_thres, num_hashes, num_buckets, random_noise_multi, random_noise_add);


//         } else if above_threshold {
//             // if left node is null, we can automatically add it to the end of the left list
//             self.accelerate_to_child(input, comp_val);
                  
//         } else { //must be between between thresholds
            
//              //we can't check the left path, so move it to the right            
//             match self.left_hash_node {
//                 Some(ref mut left_hash_node) => 
//                 {
//                     // println!("{}", "We are in the else statment");
                


//                     if (self.check_left_path(&input, upper_thres, num_hashes, num_buckets,random_noise_multi,random_noise_add).await) { 
//                         self.accelerate_to_child(input, comp_val);

//                     } else { // we've rejected the left path       
//                         self.move_child_right(input, 
//                                             upper_thres, lower_thres, 
//                                             num_hashes, num_buckets, 
//                                             random_noise_multi, random_noise_add);
                    
//                     }

//                 },
//                 None => 
//                 {
//                     match self.right_hash_node {
//                         Some(ref mut right_hash_node) => {

//                             let right_node_clone = Arc::clone(right_hash_node);
//                             let mut right_node_guard = right_node_clone.write().await;


//                             right_node_guard.descend_to_children(input, 
//                                                                 upper_thres, lower_thres, 
//                                                                 num_hashes, num_buckets, 
//                                                                 random_noise_multi, random_noise_add);
//                         },

//                         None =>
//                         {                 
//                             self.right_hash_node = Some(input);
//                             match self.right_hash_node {
//                                 Some(ref mut right_hash_node) =>{
//                                     let right_node_clone = Arc::clone(right_hash_node);
//                                     let mut right_node_guard = right_node_clone.write().await;
        
//                                      right_node_guard.make_parent();
//                                 },
//                                 None => panic!(),
//                             }       
//                         }
//                     }            
//                 }        
//             }
//         }
//     }

//     pub async fn move_child_right(&mut self, 
//                             input: Arc<RwLock<ParallelTreeNode>>, 
//                             upper_thres:f32, lower_thres:f32, 
//                             num_hashes:usize, num_buckets:usize, 
//                             random_noise_multi:&Vec<u64>, random_noise_add:&Vec<u64> ){
        
    
//         match self.right_hash_node  {
//             Some(ref mut right_hash_node) => 
//             {
//                 match self.right_hash_node {
//                     Some(ref mut right_hash_node) => {
                        
//                         let right_node_clone = Arc::clone(right_hash_node);
//                         let mut right_node_guard = right_node_clone.write().await;

                        
//                         right_node_guard.descend_to_children(input, 
//                                                             upper_thres, lower_thres, 
//                                                             num_hashes, num_buckets, 
//                                                             random_noise_multi, random_noise_add);
//                     },
//                      None => panic!(),
    
//                 }
//             },

//             None => {
//                 self.right_hash_node = Some(input);
//                 // self.right_hash_node.as_mut().unwrap().make_parent();
//                 match self.right_hash_node {
//                     Some(ref mut right_hash_node) =>{
//                         let right_node_clone = Arc::clone(right_hash_node);
//                         let mut right_node_guard = right_node_clone.write().await;

                        
//                          right_node_guard.make_parent();
//                     },
//                     None => panic!(),
//                 }
//             }
//         }
   
//     }
        
//     pub async fn accelerate_to_child(&mut self, input:Arc<RwLock<ParallelTreeNode>>, comp_value:f32){
//             match self.left_hash_node {
//                 Some(ref mut left_hash_node) => {

//                     let left_node_clone = Arc::clone(left_hash_node);
//                     let mut left_node_guard = left_node_clone.write().await;


//                     left_node_guard.accelerate_to_child(input, comp_value);
//                 },
              
//                 None => {
//                     self. left_hash_node = Some(input);
//                     match self.left_hash_node {
//                         Some(ref mut left_hash_node) => {
                            
//                             let left_node_clone = Arc::clone(left_hash_node);
//                             let mut left_node_guard = left_node_clone.write().await;
        
//                             left_node_guard.set_comp_value(comp_value);
//                         },
//                         None => panic!()
//                     }
//                 }

//             }       
//     }
    
//     pub fn check_left_path<'a>(&'a self, 
//                             input: &'a Arc<RwLock<ParallelTreeNode>>, 
//                             upper_thres:f32,     
//                             num_hashes:usize, num_buckets:usize, 
//                             random_noise_multi:&'a Vec<u64>, random_noise_add:&'a Vec<u64>  ) ->   Pin<Box<dyn Future<Output = bool> + Send + 'a>> {
//     async move {
//         let input_clone =Arc::clone(&input);
//         let mut input_guard = input_clone.read().await;
        
        
//         let self_hash = compute_hash(&self.tokens, 
//                                                 num_hashes, num_buckets, 
//                                 random_noise_multi, random_noise_add);

//         let input_hash = compute_hash(&input_guard.tokens, 
//                                                     num_hashes, num_buckets, 
//                                     random_noise_multi, random_noise_add);

//         let comp_val = compare_hash(self_hash, input_hash, num_hashes);

//         if comp_val >= upper_thres{
//             return true
//         }
        
//         match &self.left_hash_node {
//                 Some(left_hash_node) =>   {
//                     let left_node_clone = Arc::clone(left_hash_node);
//                     let mut left_node_guard = left_node_clone.write().await;


//                     left_node_guard.check_left_path(input, upper_thres, 
//                                                             num_hashes, num_buckets,
//                                                             random_noise_multi,random_noise_add).await
                                                        
//                 },
                
//                 None => {
//                     return false
//                 }
//         }
//     }.boxed()
        
//     }


//     pub async fn make_parent(&mut self){
//         self.is_parent = true;
//         self.hash_comp_value = Some(1.0);
//     }

//     pub async fn set_comp_value(&mut self, comp_val:f32){
//         self.hash_comp_value = Some(comp_val);
//     }

//     // pub fn print_tree(&mut self) -> String{
//     //     // let mut s = self.id.to_string()  + " = " + &self.hash_comp_value.unwrap().to_string();

//     //     // // if !self.left_hash_node.is_none() {
//     //     // //     s = s + {"\n-----"} + &self.left_hash_node.as_mut().unwrap().print_tree();
//     //     // // }

//     //     // // if !self.right_hash_node.is_none() {
//     //     // //     s = s + {"\n"} +  &self.right_hash_node.as_mut().unwrap().print_tree();
//     //     // // }

//     //     // s
//     // }
    
//     pub fn proccess_output(&mut self, writer: &CsvWriter){



//     }


// }