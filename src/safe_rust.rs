// use std::thread;
//
// // derive is a macro that generates the implementation of the Debug trait.
// #[derive(Debug)]
// enum Cereal {
//     Wheat,
//     Rice,
//     Oats,
//     Barley,
// }
//
// fn error_fn(){
//     let mut grains: Vec<Cereal> = vec![];
//     grains.push(Cereal::Wheat);
//     drop(grains);
//     // println!("{:?}", grains);
// }
//
//
// fn wtf_thread(){
//     let mut data = 0;
//
//     thread::spawn(|| {data = 50});
//     thread::spawn(|| {data = 1000});
//     println!("{}", data);
// }