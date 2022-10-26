/*
    array example
 */

pub fn run_array_example() {
    let one = [1, 2, 3];
    let two: [u8; 3] = [1, 2, 3]; // declare type of the array, [u8; 3] => 3 element of u8 type
    let three = [9; 3];
    let blank1 = [0; 3];
    let blank2: [u8; 3] = [0; 3]; // declared length (3, in [u8; 3]) should match with the actual assigned length of the array (3 in [0; 3])

    println!("{:?}", one); // [1, 2, 3]
    println!("{:?}", two); // [1, 2, 3]
    println!("{:?}", blank1); // [0, 0, 0]
    println!("{:?}", blank2); // [0, 0, 0]

    let arrays = [one, two, three, blank1, blank2];

    for a in arrays {
        print!("{:?}", a); // [1, 2, 3]
        for e in a {
            print!("\t{} + 10 = {}\t", e, e + 10);
        }

        let mut s = 0;
        for i in 0..a.len() {
            s += a[i];
        }
        print!("\tsum of {:?} = {}", a, s);
        println!();
    }
    /*
        [1, 2, 3]	1 + 10 = 11		2 + 10 = 12		3 + 10 = 13		sum of [1, 2, 3] = 6
        [1, 2, 3]	1 + 10 = 11		2 + 10 = 12		3 + 10 = 13		sum of [1, 2, 3] = 6
        [9, 9, 9]	9 + 10 = 19		9 + 10 = 19		9 + 10 = 19		sum of [9, 9, 9] = 27
        [0, 0, 0]	0 + 10 = 10		0 + 10 = 10		0 + 10 = 10		sum of [0, 0, 0] = 0
        [0, 0, 0]	0 + 10 = 10		0 + 10 = 10		0 + 10 = 10		sum of [0, 0, 0] = 0
     */
}