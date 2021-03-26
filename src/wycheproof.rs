extern crate serde;
extern crate serde_json;
use serde::{Deserialize, Serialize};

use crate::test_drivers::{*, self};

#[derive(Serialize, Deserialize)]
pub struct Wycheproof {
    algorithm: String,
    testGroups: Vec<WycheproofTestGroup>
}

impl Wycheproof {
    pub fn run_testcases(&self, driver: &impl test_drivers::TestDriverOps) -> bool {
        let mut pass = true;

        for test_group in &self.testGroups {
            for test in &test_group.tests {
                if driver.sig_verify(test) != true {
                    println!("Verification failed for testcase: {} comment: {}", test.tcId, test.comment);
                    pass = false;
                }
            }
        }

        return pass;
    }

    pub fn from_json_str(c : &String) -> Result<Wycheproof, serde_json::Error> {
        return match serde_json::from_str(c) {
            Ok(x) => Ok(x),
            Err(error) => Err(error)
        };
    }
}

#[derive(Serialize, Deserialize)]
struct WycheproofTestGroup {
    keysize: u16,
    mgfSha: String,
    sLen: u16,
    sha: String,
    tests: Vec<WycheproofTestCase>
}

#[derive(Serialize, Deserialize)]
pub struct WycheproofTestCase {
    tcId: u16,
    comment: String,
    pub msg: String,
    pub sig: String,
    result: String
}
