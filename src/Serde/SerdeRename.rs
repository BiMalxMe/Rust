use serde::{Deserialize,Serialize};


#[derive(Serialize, Deserialize, Debug)]
struct Datas{
    // Originally i cant use any kind of cap letter inside of the steuct but during using serde we
    //can explicitely rename to get our descired case letter
    #[serde(rename = "Meronaam")]
    name :String,
    age : u8,
    married : Option<bool>,
    desciption : String
}
 fn main(){
    let data = Datas {
        name : "Bimal Chalise".to_string(),
        age : 44,
        married : Some(false),
        desciption : " AM  a  world class bowler and so ".to_string()
    };
    let json = serde_json::to_string_pretty(&data).unwrap();
    print!("{}",json)

 }