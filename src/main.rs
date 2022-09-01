mod parse;

fn main() {

   let res = parse::main().unwrap();
   println!("{:?}", res);

}
