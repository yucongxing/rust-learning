
fn main() {
    let s = String::from("11111  hello");
    let s2 = take_ownership(s);
    let s3 = take_ownership(s2);
    
    println!("{}", first_word(&s3))
}

fn take_ownership(s:String) ->String
{
    return s;
}

fn first_word(s : &String)  -> &str
{
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate()  { 
        if item == b' '
        {
            return &s[0..i];
        }
    }
    return &s[..];
}
