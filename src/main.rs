use rustc_serialize::hex::FromHex;

fn main() {
    let d = "1c0111001f010100061a024b53535009181c".from_hex().unwrap();
    let x = "686974207468652062756c6c277320657965".from_hex().unwrap();
    let mut v = vec![];
    
    for (u_d, u_x) in d.iter().zip(x) {
        v.push(u_d ^ u_x)    
    }
    
    let s = v.iter().map(|n| format!("{:X}", n)).collect::<String>();
    
    println!("{}", s)
}