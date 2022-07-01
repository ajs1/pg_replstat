use postgres::{NoTls, Error, Client};
use std::process;


fn main() -> Result<(), Error> {
    // Connect to the database.
    let mut client = Client::connect("host=/tmp,localhost user=postgres", NoTls)?;

    let res = check_leaf_status(&mut client)?;
    let res2 = check_main_status(&mut client)?;

    match res || res2 {
        true => println!("Replication is running"),
        false => {
            eprintln!("Replication is not running");
            process::exit(1);
        }
    }
    
    Ok(())
}


fn check_main_status(client :&mut Client) -> Result<bool, Error> {
    let rows = client
        .query("SELECT state FROM pg_stat_replication", &[])?;
    match rows.len() {
        0 => Ok(false),
        _ => Ok(true)
    }        
    
}

fn check_leaf_status(client :&mut Client) -> Result<bool, Error> {
    let rows = client
        .query("SELECT status FROM pg_stat_wal_receiver", &[])?;
    match rows.len() {
        0 => Ok(false),
        _ => Ok(true)
    }        
}
