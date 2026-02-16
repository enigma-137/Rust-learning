#[derive(Debug)]
struct MyStruct {
    num: i32,
}

fn print_struct(structs: &MyStruct){
println!("{:#?}", structs);
}

fn main(){
    let my_structs: MyStruct = MyStruct{ num: 3 };
    print_struct(&my_structs);
    print_struct(&my_structs);

}



