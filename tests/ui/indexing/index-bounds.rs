//@ build-fail

fn main() {
    let _n = [64][200];
    //^~ ERROR: index out of bounds: the length is 1 but the index is 200
    let _n = [64][4_294_967_295];
    //^~ ERROR: index out of bounds: the length is 1 but the index is 4294967295
}
