use std::collections::HashMap;
use const_random::const_random;
use rand::Rng;


const SBOX: [u8; 256] = const_random!([u8; 256]);
//const NUMBER_ROUNDS: u8 = 32;


fn encode_1_round(plain_text: &mut [u8] , key: &[u8]) {
    let mut temp: u8 = plain_text[0];
    for j in 0..4 {
        temp = temp + key[j];
        temp = SBOX[temp as usize] + plain_text[(j+1) % plain_text.len()];
        temp = (temp << 1) | (temp >> 7);
        plain_text[(j+1) % plain_text.len()] = temp;
    }
}

fn encode_2_rounds(plain_text: &mut [u8] , key: &[u8]) {    
    let mut temp: u8 = plain_text[0];
    for _ in 0..2 {
        for j in 0..4 {
            temp = temp + key[j];
            temp = SBOX[temp as usize] + plain_text[(j+1) % plain_text.len()];
            temp = (temp << 1) | (temp >> 7);
            plain_text[(j+1) % plain_text.len()] = temp;
        }
    }
}

//UNUSED FUNCTIONS

/* fn _encode_many_rounds(plain_text: &mut [u8] , key: &[u8]) {    
    let mut temp: u8 = plain_text[0];
    for _ in 0.._NUMBER_ROUNDS {
        for j in 0..8 {
            temp = temp + key[j];
            temp = SBOX[temp as usize] + plain_text[(j+1) % plain_text.len()];
            temp = (temp << 1) | (temp >> 7);
            plain_text[(j+1) % plain_text.len()] = temp;
        }
    }
}

fn _decode_many_rounds(cipher_text: &mut [u8], key: &[u8]) {
    let mut top: u8;
    let mut bottom: u8;

    for _ in 0.._NUMBER_ROUNDS {
        let mut j = 7;
        loop {
            top = cipher_text[j] + key[j];
            top = SBOX[top as usize];
            bottom = cipher_text[(j + 1) % cipher_text.len()];
            bottom = (bottom >> 1) | (bottom << 7);
            cipher_text[(j+1) % cipher_text.len()] = bottom - top;
            if j == 0 {
                break;
            }
            j -= 1;
        }
    }
}

fn _treyfer_encode(plain: &mut [u8], key: &[u8]) {
    if plain.len()%8 != 0 {
        panic!("Array length needs to be divisable by 8")
    }

    for i in 0..plain.len()/8 {
        _encode_many_rounds(&mut plain[i*8..i*8+8], key);
    }
}

fn _treyfer_decode(cipher: &mut [u8], key: &[u8]) {
    if cipher.len()%8 != 0 {
        panic!("Array length needs to be divisable by 8")
    }

    for i in 0..cipher.len()/8 {
        _decode_many_rounds(&mut cipher[i*8..i*8+8], key);
    }
} */

fn hashmap_creation(size: usize, cipher_key: &[u8]) -> HashMap<[u8; 4], ([u8; 4],[u8; 4],[u8; 4])> {
    let mut rng = rand::thread_rng();
    let mut map: HashMap<[u8; 4], ([u8; 4],[u8; 4],[u8; 4])> = HashMap::new();

    while map.len() < size {
        let mut map_key: [u8; 4] = [0; 4];
        rng.fill(&mut map_key);

        let mut value2 = map_key.clone();
        encode_1_round(&mut value2, cipher_key);
        let mut value3 = map_key.clone();
        encode_2_rounds(&mut value3, cipher_key);

        map.entry(map_key).or_insert((map_key,value2,value3));

    }
    map
}


fn main() {
    let key = "dead".as_bytes();

    println!("Creating map!");
    let map = hashmap_creation(2_u32.pow(16) as usize, key);
    println!("Map created!\nLooking for values");

    for (_, (value1,value2,value3)) in map.iter() {
        match map.get(value2) {
            None => continue,
            Some((bvalue1,bvalue2,_)) => {
                println!("FOUND A VALUE BICH");

                if value3 == bvalue2 {
                    println!("SLID PAIR FOUND");

                    print!("(P,C,CC): (");
                    for c in value1 {
                        print!("{} ", *c);
                    }
                    print!("), ");

                    print!("(");
                    for c in value2 {
                        print!("{} ", *c);
                    }
                    print!(")");

                    print!("(");
                    for c in value3 {
                        print!("{} ", *c);
                    }
                    print!(")");
                    println!("");

                    print!("(P',C'): (");
                    for c in bvalue1 {
                        print!("{} ", *c);
                    }
                    print!("), ");

                    print!("(");
                    for c in bvalue2 {
                        print!("{} ", *c);
                    }
                    print!(")");
                    println!("");
                    
                    continue;
                }
            }   
        }
    }
    //FOR PRINTING THE WHOLE HASHMAP
/*     let mut val = 0;
    for (_, (value1,value2,value3)) in map.iter() {
        print!("{}: (", val);
        for c in value1 {
            print!("{} ", *c);
        }
        print!(", ");

        for c in value2 {
            print!("{} ", *c);
        }
        print!(", ");

        for c in value3 {
            print!("{} ", *c);
        }
        print!(")\n");
        val += 1;
    }
    println!("No values? 0.0") */
}