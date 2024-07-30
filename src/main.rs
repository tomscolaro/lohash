mod tree;
mod paralleltree;
mod record;
mod hash;
mod writer;
mod vectortree;

use hash::compute_hash;
use record::Record;
// use tree::TreeNode;
// use tree::Tree;


use clap::Parser;
use vectortree::VecNode;
use vectortree::VecTree;
use writer::CsvWriter;
use std::fs::File;
use chrono::Utc;


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// string of input file name 
    #[arg(short, long)]
    input_file: String,

    /// string of output file name
    #[arg(short, long)]
    output_file: String,

    /// num of hashes
    #[arg(long, default_value_t = 60)]
    hash_num: u8,

    /// num of modulo buckets
    #[arg(short, long, default_value_t = 25)]
    bucket_num: u8,

    /// upper threshold percentage
    #[arg(short, long, default_value_t = 0.5)]
    upper_threshold: f32,

    /// lower threshold percentage
    #[arg(short, long, default_value_t = 0.3)]
    lower_threshold: f32,

    #[arg(short, long, default_value_t = false)]
    print_tree: bool,

}


fn main() {
    //TODO: argument parser
    let args = Args::parse();

    let hash_num:usize = args.hash_num.into();  
    let bucket_num:usize = args.bucket_num.into(); // should be a prime number
    let upper_thres = args.upper_threshold;
    let lower_thres = args.lower_threshold;    
    let input_file = File::open(args.input_file);
    
    
    let mut rdr = csv::Reader::from_reader(input_file.unwrap());
    let mut search_tree = Box::new(VecTree::new(hash_num,
                                upper_thres, lower_thres ));
    let mut iter = 0;
    let mut prev = Utc::now().time();


    // println!("Reading and Sorting Tree...");

    // for result in rdr.deserialize() {
    //     let record: Record = result.unwrap();
    //     let toks = hash::shingle_string(record.str.clone(), 2);
    //     let row = Box::new(TreeNode{
    //                                 id: record.id,
    //                                 idx: record.idx,
    //                                 data: record.str,
    //                                 data_name: record.name,
    //                                 tokens: toks,
    //                                 hash_comp_value: None,
    //                                 is_parent:false,
    //                                 left_hash_node: None,
    //                                 right_hash_node: None
    //                                 });
    //     if (iter % 500) == 0{
    //         let curr = Utc::now().time();
           
    //         println!("Iteration: {:} - {:#?}", iter, (curr - prev).num_seconds() );
    //         prev = curr
    //     }       
        
    //     search_tree.place_node(row);

    //     iter += 1;
    // }

    // // search_tree.print_tree();
    // if args.print_tree {
    //     search_tree.print_tree();
    // }

    // println!("Tree has been created. Processing output...");

    // let output_vec = search_tree.proccess_output();
    // let writer = CsvWriter::new(args.output_file);
    // let _ = writer.write_output_file_from_csv(output_vec);


    let mut node_vec : Vec<Vec<VecNode>> = vec![];
    println!("Reading and Sorting Tree...");
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();
        
        let toks = hash::shingle_string(record.str.clone(), 2);
        // let toks_copy = hash::shingle_string(record.str.clone(), 2);
        let h = compute_hash(&toks, hash_num, bucket_num, &search_tree.random_noise_multi, &search_tree.random_noise_add);

        let mut row = VecNode{
                                    id: record.id,
                                    idx: record.idx,
                                    data: record.str,
                                    data_name: record.name,
                                    hash: h, 
                                    hash_comp_value: None
                                
                                    };

        match search_tree.place_node(&mut row, &mut node_vec) 
        {
            (-1.0, 1.0) => {
                    row.hash_comp_value = Some(1.0);
                    node_vec.push(vec![row])
            
            
            },
            (i, val) => {
                    row.hash_comp_value = Some(val);
                    node_vec[ i as usize ].push(row)
            }

        };

        if (iter % 500) == 0{
            let curr = Utc::now().time();
           
            println!("Iteration: {:} - {:#?}", iter, (curr - prev).num_seconds() );
            prev = curr
        }       
        iter += 1;
        
    }



    println!("Tree has been created. Processing output...");

    let output_vec = search_tree.process_output(&mut node_vec);
    let writer = CsvWriter::new(args.output_file);
    let _ = writer.write_output_file_from_vector_nodes(output_vec);

    // ############ todo: PARALLEL IMPLEMENTATION ######################
    // let mut search_tree = ParallelTree::new(hash_num, bucket_num, 
    //     upper_thres, lower_thres );


    // // let mut test = vec![];
    // // let mut iter = 0;
    // let mut par_read: Vec<ParallelTreeNode> = vec![];

    // for result in rdr.deserialize() {
    // let record: Record = result.unwrap();
    // let toks = hash::shingle_string(record.str, 2);
    // let row = ParallelTreeNode{
    //             id: record.id,
    //             tokens: toks,
    //             hash_comp_value: None,
    //             is_parent:false,
    //             left_hash_node: None,
    //             right_hash_node: None
    //             };


    // par_read.push(row);

    // }




    // // search_tree.print_tree();
    // search_tree.place_node();




}
