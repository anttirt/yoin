pub mod mast;
pub mod ir;
pub mod op;

fn main() {
    let samples: Vec<(&[u8], [u8; 4])> = vec![(b"apr", [0, 0, 3, 0]),
                                              (b"aug", [0, 0, 3, 1]),
                                              (b"dec", [0, 0, 3, 1]),
                                              (b"feb", [0, 0, 2, 8]),
                                              (b"feb", [0, 0, 2, 9]),
                                              (b"jan", [0, 0, 3, 1]),
                                              (b"jul", [0, 0, 3, 0]),
                                              (b"jun", [0, 0, 3, 1])];
    let samples = samples.into_iter()
        .map(|(x, bytes)| {
            let out: i32 = unsafe { ::std::mem::transmute(bytes) };
            (x, out)
        });
    let m = mast::Mast::build(samples);

    println!("build MAST and interpret");
    for out in m.run(b"feb").unwrap() {
        let buf : [u8; 4] = unsafe { ::std::mem::transmute(out) };
        println!("{:?}", buf);
    }

    println!("build IR and interpret");
    for out in ir::run(&m, b"feba").unwrap() {
        let (n, substr) = out;
        let buf : [u8; 4] = unsafe { ::std::mem::transmute(n) };
        println!("{}: {:?}", String::from_utf8_lossy(substr), buf);
    }
}