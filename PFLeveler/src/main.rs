use PathFinder::pf_table;

fn main() {
    let filename = String::from("E:\\dev\\PFLeveler\\cfg\\PathFinder.db");
    let _table = pf_table::init_pf_table(&filename);
    
    // Now you can access the table anywhere with:
    // pf_table::PF_TABLE.get().unwrap()
}
