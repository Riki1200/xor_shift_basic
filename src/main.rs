
struct XorShift32 {
    xor_value: u32
}

fn xor_shift(mut state:  XorShift32) -> u32 {
    let mut to_shift = state.xor_value;
    to_shift ^= to_shift <<  13;
    to_shift ^= to_shift >>  17;
    to_shift ^= to_shift << 5;


    state.xor_value = to_shift;
    state.xor_value
}


struct XorShift64State {
    to_shift_value: u64
}

fn  xor_shift64(mut state: XorShift64State) -> u64 {
    let mut x = state.to_shift_value;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    state.to_shift_value = x;
    state.to_shift_value
}


struct XorShift128State {
    x: [u32; 4],
}


fn xor_shift128(mut state: XorShift128State) -> u32 {

    let  mut  t = state.x[3];
    let s = state.x[0];

    state.x[3] = state.x[2];
    state.x[2] = state.x[1];
    state.x[1] = s;

    t ^= t << 11;
    t ^= t >> 8;
    state.x[0] = t ^ s ^ (s >> 19);
    state.x[0]
}

fn main() {

    //1351845 for 5
    //1622214 for 6

    let  xor32 = XorShift32 {
        xor_value: 1
    };

    let xor_value32 = xor_shift(xor32);

    println!("xor shift value {}", xor_value32);


    let xor64 = XorShift64State {
        to_shift_value: 1
    };


    let xor_value64 = xor_shift64(xor64);

    println!("xor shift value {}", xor_value64);


    let xor_128 = XorShift128State {
        x: [1,1,1,1]
    };

    let xor_value_128 = xor_shift128(xor_128);

    println!("the value is {}", xor_value_128);


}


