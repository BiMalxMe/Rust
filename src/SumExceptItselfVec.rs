// program to get the sum inside the vec
//E.G
// if vec is [a,b,c] now a=b+c,b=a+c and c= a+b

fn main(){
    let newdata = vec![10,2,3,10];
    let getsum_except_itself = getval(newdata);
    println!("{:?}",getsum_except_itself)
}
fn getval( data : Vec<u8>) -> Vec<u8>{
    let mut newvec = Vec::new();
    for x in 0..data.len(){
        let mut sum = 0;

    for (i,l) in data.iter().enumerate(){
        if x==i{
        }else{
        sum = sum + l;
    }

}
newvec.insert(x, sum);}
newvec
}


