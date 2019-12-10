fn intcodeCompute(input: &mut [i32], inputID: i32) -> i32 {
    let limit = input.len();
    let mut address = 0;
    let mut diagnosticCode = 0;
    while address < limit {
        let code = input[address];
        let opcode = code % 100;
        match opcode {
            1 => opcode1(input, address, code / 100),
            2 => opcode2(input, address, code / 100),
            3 => opcode3(input, address, inputID),
            4 => diagnosticCode = opcode4(input, address),
            5 => address = opcode5(input, address, code / 100),
            6 => address = opcode6(input, address, code / 100),
            7 => opcode7(input, address, code / 100),
            8 => opcode8(input, address, code / 100),
            _ => break
        }

        match opcode {
            1 | 2 | 7 | 8 => address += 4,
            3 | 4  => address += 2,
            _ => address += 0
        }
    }
    return diagnosticCode;
}

fn getParam(input: &mut [i32], mode: i32, address: usize) -> i32 {
    let mut param = input[address];
    if mode == 0 {
        param = input[param as usize];
    }
    param
}

fn saveAtDestination(input: &mut [i32], address: usize, value: i32) {
    let mut destination = input[address];
    input[destination as usize] = value;
}

fn opcode1(input: &mut [i32], address: usize, code: i32) {
    let mut modes = code;
    let mut destination = input[address + 3];
    let firstParam = getParam(input, modes % 10, address + 1);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2);
    modes /= 10;
    saveAtDestination(input, address + 3, firstParam + secondParam);
}

fn opcode2(input: &mut [i32], address: usize, code: i32) {
    let mut modes = code;
    let mut destination = input[address + 3];
    let firstParam = getParam(input, modes % 10, address + 1);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2);
    modes /= 10;
    saveAtDestination(input, address + 3, firstParam * secondParam);
}

fn opcode3(input: &mut [i32], address: usize, inputID: i32) {
    saveAtDestination(input, address + 1, inputID)
}

fn opcode4(input: &mut [i32], address: usize) -> i32 {
    let destination = input[address + 1] as usize;
    return input[destination];
}

fn opcode5(input: &mut [i32], address: usize, code: i32) -> usize {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1);
    modes /= 10;
    if firstParam != 0 {
        let mut newAddress = input[address + 2] as usize;
        if modes % 10 == 1 {
            return newAddress;
        }
        return input[newAddress] as usize;
    }
    return address + 3;
}

fn opcode6(input: &mut [i32], address: usize, code: i32) -> usize {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1);
    modes /= 10;
    if firstParam == 0 {
        let mut newAddress = input[address + 2] as usize;
        if modes % 10 == 1 {
            return newAddress;
        }
        return input[newAddress] as usize;
    }
    return address + 3;
}

fn opcode7(input: &mut [i32], address: usize, code: i32) {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2);
    saveAtDestination(input, address + 3, 0);
    if firstParam < secondParam {
        saveAtDestination(input, address + 3, 1);
    }
}

fn opcode8(input: &mut [i32], address: usize, code: i32) {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2);
    saveAtDestination(input, address + 3, 0);
    if firstParam == secondParam {
        saveAtDestination(input, address + 3, 1);
    }
}


fn getCopy(input: &[i32]) -> Vec<i32> {
    let mut copy = vec![0; input.len()];
    copy.clone_from_slice(input);
    return copy;
}


fn main() {
    let input = [3, 225, 1, 225, 6, 6, 1100, 1, 238, 225, 104, 0, 1102, 79, 14, 225, 1101, 17, 42, 225, 2, 74, 69, 224, 1001, 224, -5733, 224, 4, 224, 1002, 223, 8, 223, 101, 4, 224, 224, 1, 223, 224, 223, 1002, 191, 83, 224, 1001, 224, -2407, 224, 4, 224, 102, 8, 223, 223, 101, 2, 224, 224, 1, 223, 224, 223, 1101, 18, 64, 225, 1102, 63, 22, 225, 1101, 31, 91, 225, 1001, 65, 26, 224, 101, -44, 224, 224, 4, 224, 102, 8, 223, 223, 101, 3, 224, 224, 1, 224, 223, 223, 101, 78, 13, 224, 101, -157, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 3, 224, 1, 224, 223, 223, 102, 87, 187, 224, 101, -4698, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 4, 224, 1, 223, 224, 223, 1102, 79, 85, 224, 101, -6715, 224, 224, 4, 224, 1002, 223, 8, 223, 1001, 224, 2, 224, 1, 224, 223, 223, 1101, 43, 46, 224, 101, -89, 224, 224, 4, 224, 1002, 223, 8, 223, 101, 1, 224, 224, 1, 223, 224, 223, 1101, 54, 12, 225, 1102, 29, 54, 225, 1, 17, 217, 224, 101, -37, 224, 224, 4, 224, 102, 8, 223, 223, 1001, 224, 3, 224, 1, 223, 224, 223, 1102, 20, 53, 225, 4, 223, 99, 0, 0, 0, 677, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1105, 0, 99999, 1105, 227, 247, 1105, 1, 99999, 1005, 227, 99999, 1005, 0, 256, 1105, 1, 99999, 1106, 227, 99999, 1106, 0, 265, 1105, 1, 99999, 1006, 0, 99999, 1006, 227, 274, 1105, 1, 99999, 1105, 1, 280, 1105, 1, 99999, 1, 225, 225, 225, 1101, 294, 0, 0, 105, 1, 0, 1105, 1, 99999, 1106, 0, 300, 1105, 1, 99999, 1, 225, 225, 225, 1101, 314, 0, 0, 106, 0, 0, 1105, 1, 99999, 107, 226, 226, 224, 1002, 223, 2, 223, 1006, 224, 329, 101, 1, 223, 223, 1108, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 344, 101, 1, 223, 223, 7, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 359, 101, 1, 223, 223, 108, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 374, 101, 1, 223, 223, 8, 226, 677, 224, 1002, 223, 2, 223, 1006, 224, 389, 101, 1, 223, 223, 1108, 226, 226, 224, 102, 2, 223, 223, 1006, 224, 404, 101, 1, 223, 223, 1007, 677, 677, 224, 1002, 223, 2, 223, 1006, 224, 419, 101, 1, 223, 223, 8, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 434, 1001, 223, 1, 223, 1008, 226, 226, 224, 102, 2, 223, 223, 1005, 224, 449, 1001, 223, 1, 223, 1008, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 464, 101, 1, 223, 223, 1107, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 479, 101, 1, 223, 223, 107, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 494, 1001, 223, 1, 223, 1107, 226, 677, 224, 1002, 223, 2, 223, 1005, 224, 509, 101, 1, 223, 223, 1108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 524, 101, 1, 223, 223, 7, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 539, 101, 1, 223, 223, 108, 677, 677, 224, 1002, 223, 2, 223, 1005, 224, 554, 101, 1, 223, 223, 8, 677, 226, 224, 1002, 223, 2, 223, 1005, 224, 569, 1001, 223, 1, 223, 1008, 677, 677, 224, 102, 2, 223, 223, 1006, 224, 584, 101, 1, 223, 223, 107, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 599, 1001, 223, 1, 223, 7, 226, 677, 224, 102, 2, 223, 223, 1005, 224, 614, 101, 1, 223, 223, 1007, 226, 226, 224, 1002, 223, 2, 223, 1005, 224, 629, 101, 1, 223, 223, 1107, 677, 226, 224, 1002, 223, 2, 223, 1006, 224, 644, 101, 1, 223, 223, 108, 226, 677, 224, 102, 2, 223, 223, 1006, 224, 659, 101, 1, 223, 223, 1007, 677, 226, 224, 102, 2, 223, 223, 1006, 224, 674, 101, 1, 223, 223, 4, 223, 99, 226];
    let mut inputCopy = getCopy(&input); // for part 1
    let mut diagnosticCode = intcodeCompute(&mut inputCopy, 1);
    println!("diagnostic code from part 1 {}", diagnosticCode);
    inputCopy = getCopy(&input);
    diagnosticCode = intcodeCompute(&mut inputCopy, 5);
    println!("diagnostic code from part 2 {}", diagnosticCode);
}
