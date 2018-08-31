fn main() {
    //Set RUST_BACKTRACE=1 to view backtrace.panic!
    //In PowerShell $env:RUST_BACKTRACE=1
    let v = vec![1,2,3];

    v[99];
}
