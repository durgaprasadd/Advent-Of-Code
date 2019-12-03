fn intcodeCompute(input:&mut [i32]){
    let limit = input.len();
    let mut address = 0;
    while address <limit{
        let opcode = input[address];
        if opcode == 1{
            opcode1(input, address);
        }
        if opcode == 2{
            opcode2(input, address)
        }
        if opcode == 99{
            break;
        }
        address += 4;
    }
}

fn opcode1(input:&mut [i32], address:usize){
    let destination = input[address +3] as usize;
    let firstParamAddress =input[address +1] as usize;
    let secondParamAddress = input[address +2] as usize;
    input[destination] = input[firstParamAddress]+input[secondParamAddress]
}

fn opcode2(input:&mut [i32], address:usize){
    let destination = input[address +3] as usize;
    let firstParamAddress =input[address +1] as usize;
    let secondParamAddress = input[address +2] as usize;
    input[destination] = input[firstParamAddress]*input[secondParamAddress]
}

fn getNounAndVerb(input:&[i32],value:i32) -> (i32,i32){
    for noun in 0..100{
        for verb in 0..100{
            let mut copy = getCopy(input);
            copy[1] = noun;
            copy[2] = verb;
            intcodeCompute(&mut copy);
            if value == copy[0] {
                return (noun,verb);
            }
        }
    }
    return (0,0);
}

fn getCopy(input:&[i32])->Vec<i32>{
    let mut copy = vec![0;input.len()];
    copy.clone_from_slice(input);
    return copy;
}


fn main() {
    let input = [1,12,2,3,1,1,2,3,1,3,4,3,1,5,0,3,2,6,1,19,1,5,19,23,1,13,23,27,1,6,27,31,2,31,13,35,1,9,35,39,2,39,13,43,1,43,10,47,1,47,13,51,2,13,51,55,1,55,9,59,1,59,5,63,1,6,63,67,1,13,67,71,2,71,10,75,1,6,75,79,1,79,10,83,1,5,83,87,2,10,87,91,1,6,91,95,1,9,95,99,1,99,9,103,2,103,10,107,1,5,107,111,1,9,111,115,2,13,115,119,1,119,10,123,1,123,10,127,2,127,10,131,1,5,131,135,1,10,135,139,1,139,2,143,1,6,143,0,99,2,14,0,0];
    let mut inputCopy = getCopy(&input); // for part 1
    intcodeCompute(&mut inputCopy);
    println!("result for part 1 is {}",inputCopy[0]);

    inputCopy = getCopy(&input); //for part 2
    let (noun,verb) = getNounAndVerb(&inputCopy,19690720);
    println!("result for part 2 is {}",noun*100+verb);
}
