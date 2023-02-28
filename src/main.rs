// use std::thread;

// derive is a macro that generates the implementation of the Debug trait.
// #[derive(Debug)]
// enum Cereal {
//     Wheat,
//     Rice,
//     Oats,
//     Barley,
// }


fn main() {
    // let arabic = "محمد سيف";
    // let english = "Muhammad Saif";
    // let list_of_lang = [arabic, english];
    // for lang in list_of_lang {
    //     // the ampersand is borrows lang for read only access.
    //     println!("{}", &lang);
    // }

    basic_text_processing()
}


fn basic_text_processing() {
    let raw_data = "\
    common name,length (cm)
    Little penguin,33
    Yellow-eyed penguin,65
    Fiordland penguin,60
    Invalid,data
    ";

    let records = raw_data.lines();

    for (i, record) in records.enumerate() {
        // skip the header and empty lines.
        if i == 0 || record.trim().is_empty() {
            continue;
        }

        // split the record into fields.
        let fields: Vec<_> = record
            .split(',')
            .map(|field| field.trim())
            .collect();

        // cfg! is a macro that returns true if the given configuration is set.
        if cfg!(debug_assertions){
            eprint!("debug: {:?} -> {:?}\n", record, fields)
        }

        let name = fields[0];
        // parse the length field as a float. ::<f32> is a type annotation.
        if let Ok(length) = fields[1].parse::<f32>() {
            println!("{}, {}cm\n", name, length)
        }
    }
}

// fn error_fn(){
//     let mut grains: Vec<Cereal> = vec![];
//     grains.push(Cereal::Wheat);
//     drop(grains);
//     println!("{:?}", grains);
// }

// fn wtf_thread(){
//     let mut data = 0;
//
//     thread::spawn(|| {data = 50});
//     thread::spawn(|| {data = 1000});
//
//     println!("{}", data);
// }