use std::fs;

mod wycheproof;
mod test_drivers;

use crate::test_drivers::*;
use crate::wycheproof::*;

fn main() -> std::io::Result<()> {
    let test_driver = KeyctlTestDriver;
    let fpath = "/home/varad/devel/wycheproof-rsa-vectors/rsa_pss_2048_sha256_mgf1_0_test.json";
    let contents = fs::read_to_string(fpath)?;

    let wp: Wycheproof = Wycheproof::from_json_str(&contents)?;
    wp.run_testcases(&test_driver);

    return Ok(());
}
