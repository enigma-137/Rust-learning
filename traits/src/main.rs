
struct Solidity {
    version: String
}
struct Vyper {
    version: String
}

#![allow (unused)]
trait Compiler {
    fn compile(&self, file_path: &str) -> String;

}
impl Compiler for Solidity {
    fn compile(&self, file_path: &str) -> String{
        format!("Solc {}", file_path)
    }
}

impl Compiler for Vyper {
    fn compile(&self, file_path: &str) -> String{
        format!("vyper {}", file_path)
    }
}


fn compiler(lang: impl Compiler, file_path: &str ) -> String{
lang.compile(file_path)
}

fn main() {
    let sol = Solidity{version: "0.8".to_string()};
    let vyp = Vyper{version: "0.8".to_string()};

    println!("Sol compile: {}", sol.compile("hello.sol"));
     println!("Vyper compile: {}", vyp.compile("hello.vyp"));

}
