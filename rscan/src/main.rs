use rscan::NetworkList;

fn main() -> Result<(), String> {
    let target_networks = NetworkList::scan()?
        .sort_closest()
        .into_vec();

    for n in target_networks {
        println!("{n}");
    }
    Ok(())
}