pub trait Tester {
    fn test(&self, file_path: &str) -> String;
}

pub struct Foundry {
    pub version: String,
}

pub struct Cargo {
    pub version: String,
}

impl Tester for Foundry {
    fn test(&self, file_path: &str) -> String {
        format!("Test for foundry: {}", file_path)
    }
}

impl Tester for Cargo {
    fn test(&self, file_path: &str) -> String {
        format!("Test for cargo: {}", file_path)
    }
}

pub fn test(tester: &impl Tester, file_path: &str) -> String {
    tester.test(file_path)
    
}
