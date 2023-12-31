use std::fs::File;
use std::io::Read;
use serde_json::Value;

pub fn json2csv(input: &str, output:&str) {
    let mut b = String::new();
    
    let mut f = File::open(input).expect("file not found");
    f.read_to_string(&mut b).expect("something went wrong reading the file");

    let contents: Value = serde_json::from_str(&b).unwrap();
    
    let mut writer = csv::Writer::from_path(output).unwrap();
    
    for (i, content) in contents.as_array().unwrap().iter().enumerate() {
        
        if i == 0 {
            let mut row = vec![];
            for (key,_) in content.as_object().unwrap().iter() {
                row.push(key.as_str());
            } 
            writer.write_record(&row).expect("can not write record");
        }

        let mut row = vec![];
        for (_,value) in content.as_object().unwrap().iter() {
            row.push(value.as_str().unwrap());

        } 

        writer.write_record(row).expect("can not write record");
    }

    writer.flush().expect("can not flush the writer");

}
