fn main() {
    // let ins = "ldr r0, [r1, #10]";
    let ins = "ldr r0, [r1]";
    // let ins = "ldr r0, [r1], #10";

    let (one, part) = ins.split_once(" ").unwrap();

    println!("{one}");

    let parts = part.split(",").map(|x| x.trim()).collect::<Vec<_>>();
    if parts.len() < 2 {
        panic!("Fuck");
    }
    println!("{r}", r = parts.len());
    let mut index = false;
    let mut val = 0u16;

    let rd = parts[0];
    let mut rn = &parts[1][1..];
    // let mut rn = &rn[1..];

    if let Some(r) = parts.get(2) {
        if !r.ends_with("]") {
            index = true;
            rn = &rn[..rn.len() - 1];
            val = r[1..].parse::<u16>().unwrap();
        } else {
            val = r[1..r.len() - 1].parse::<u16>().unwrap();
        }
    } else {
        rn = &rn[..rn.len() - 1];
    }

    let a = "abc";
    let b = "abcd";
    let c = "abcde";

    // println!("rd: {rd}");
    // println!("post-index: {index}");
    // println!("rn: {rn}");
    // println!("offset: {val}");

    println!("{a}", a = shorten(a));
    println!("{b}", b = shorten(b));
    println!("{c}", c = shorten(c));
}

fn shorten(s: &str) -> &str {
    if s.len() > 3 {
        &s[..3]
    } else {
        s
    }
}
