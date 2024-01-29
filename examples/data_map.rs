use x7cf_cty_pl_lib::util::data_map;
use x7cf_cty_pl_lib::util::data_map::DataMap;

fn main() {
    let mut data_map = data_map::z0::DataMap::default();

    data_map.set_record("abc", 1004);
    data_map.set_record("aba", 10);

    println!("Abc: {:?}", data_map.get_record("abc"));
    println!("Aba: {:?}", data_map.get_record("aba"));
}