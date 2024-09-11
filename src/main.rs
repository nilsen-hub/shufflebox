// shuffles a deck of cards in a predictable way using a seeded PRNG
use rand::{Rng, SeedableRng};
use std::io;

fn main() {
     
    let lut = ["K-A","K-2","K-3","K-4","K-5","K-6","K-7","K-8","K-9","K-10","K-J","K-Q","K-K",
               "R-A","R-2","R-3","R-4","R-5","R-6","R-7","R-8","R-9","R-10","R-J","R-Q","R-K",
               "H-A","H-2","H-3","H-4","H-5","H-6","H-7","H-8","H-9","H-10","H-J","H-Q","H-K",
               "S-A","S-2","S-3","S-4","S-5","S-6","S-7","S-8","S-9","S-10","S-J","S-Q","S-K"];
    let mut target      = [0; 52];
    let mut source      = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,
                           16,17,18,19,20,21,22,23,24,25,26,27,28,
                           29,30,31,32,33,34,35,36,37,38,39,40,41,
                           42,43,44,45,46,47,48,49,50,51];
    println!("Please input your seed number: ");
    let mut seed = String::new();

    io::stdin()
        .read_line(&mut seed)
        .expect("Failed to read line");

    let mut seed: u64 = match seed.trim().parse(){
        Ok(num) => num,
        Err(_)  => todo!()
    };

    let mut val:     usize;
    let mut seus:    usize;
    let mut counter: usize;
    //let mut seed           = 69;
    let mut array_pointer  = 0;
    let mut left_in_deck   = 52;

    while left_in_deck > 0 {
        let mut useed = rand_chacha::ChaCha8Rng::seed_from_u64(seed);
        seed = useed.gen::<u64>();
        seus = seed as usize;
        val = seus % left_in_deck;
        left_in_deck -= 1;
        target[array_pointer] = source[val];
        array_pointer = array_pointer + 1;
        loop {
            
            if val == 51{
                break;
            }

            counter = val + 1;
            source[val] = source[counter];
            val += 1;
        }
    }
    
    for el in target{
        println!("{}", lut[el]);
    }
    
    
}

