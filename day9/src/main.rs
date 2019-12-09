fn intcodeCompute(input: &mut [i64], inputID: i64) -> i64 {
    let limit = input.len();
    let mut address = 0;
    let mut diagnosticCode = 0;
    let mut relativeBase = 0;
    while address < limit {
        let code = input[address];
        let opcode = code % 100;
        if opcode == 1 {
            opcode1(input, address, code / 100, relativeBase);
            address += 4;
        }
        if opcode == 2 {
            opcode2(input, address, code / 100, relativeBase);
            address += 4;
        }
        if opcode == 3 {
            opcode3(input, address, inputID, code / 100, relativeBase);
            address += 2;
        }
        if opcode == 4 {
            diagnosticCode = opcode4(input, address, code / 100, relativeBase);
            address += 2;
        }
        if opcode == 5 {
            address = opcode5(input, address, code / 100, relativeBase);
        }
        if opcode == 6 {
            address = opcode6(input, address, code / 100, relativeBase);
        }
        if opcode == 7 {
            opcode7(input, address, code / 100, relativeBase);
            address += 4;
        }
        if opcode == 8 {
            opcode8(input, address, code / 100, relativeBase);
            address += 4;
        }
        if opcode == 9 {
            relativeBase += opcode9(input, address, code / 100, relativeBase);
            address += 2;
        }
        if opcode == 99 {
            break;
        }
    }
    return diagnosticCode;
}

fn getParam(input: &mut [i64], mode: i64, address: usize, relativeBase: i64) -> i64 {
    let mut param = input[address];
    if mode == 0 {
        param = input[param as usize];
    }
    if mode == 2 {
        param = input[(param + relativeBase) as usize];
    }
    param
}

fn saveAtDestination(input: &mut [i64], address: usize, mode: i64, relativeBase: i64, value: i64) {
    let mut destination = input[address];
    if mode == 2 {
        destination = destination + relativeBase;
    }
    input[destination as usize] = value;
}

fn opcode1(input: &mut [i64], address: usize, code: i64, relativeBase: i64) {
    let mut modes = code;
    let mut destination = input[address + 3];
    let firstParam = getParam(input, modes % 10, address + 1, relativeBase);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2, relativeBase);
    modes /= 10;
    saveAtDestination(input, address + 3, modes % 10, relativeBase, firstParam + secondParam);
}

fn opcode2(input: &mut [i64], address: usize, code: i64, relativeBase: i64) {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1, relativeBase);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2, relativeBase);
    modes /= 10;
    saveAtDestination(input, address + 3, modes % 10, relativeBase, firstParam * secondParam);
}

fn opcode3(input: &mut [i64], address: usize, inputID: i64, code: i64, relativeBase: i64) {
    saveAtDestination(input,address+1,code%10,relativeBase,inputID);
}

fn opcode4(input: &mut [i64], address: usize, code: i64, relativeBase: i64) -> i64 {
    let mut modes = code;
    let mut output = input[address + 1];
    if modes % 10 == 0 {
        output = input[output as usize];
    }
    if modes % 10 == 2 {
        output = input[(output + relativeBase) as usize];
    }
    return output;
}

fn opcode5(input: &mut [i64], address: usize, code: i64, relativeBase: i64) -> usize {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1, relativeBase);
    modes /= 10;
    if firstParam != 0 {
        let mut newAddress = input[address + 2];
        if modes % 10 == 1 {
            return newAddress as usize;
        }
        if modes % 10 == 2 {
            return input[(newAddress + relativeBase) as usize] as usize;
        }
        return input[newAddress as usize] as usize;
    }
    return address + 3;
}

fn opcode6(input: &mut [i64], address: usize, code: i64, relativeBase: i64) -> usize {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1, relativeBase);
    modes /= 10;
    if firstParam == 0 {
        let mut newAddress = input[address + 2];
        if modes % 10 == 1 {
            return newAddress as usize;
        }
        if modes % 10 == 2 {
            return input[(newAddress + relativeBase) as usize] as usize;
        }
        return input[newAddress as usize] as usize;
    }
    return address + 3;
}

fn opcode7(input: &mut [i64], address: usize, code: i64, relativeBase: i64) {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1, relativeBase);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2, relativeBase);
    modes /= 10;
    saveAtDestination(input,address+3,modes%10,relativeBase,0);
    if firstParam < secondParam {
        saveAtDestination(input,address+3,modes%10,relativeBase,1);
    }
}

fn opcode8(input: &mut [i64], address: usize, code: i64, relativeBase: i64) {
    let mut modes = code;
    let firstParam = getParam(input, modes % 10, address + 1, relativeBase);
    modes /= 10;
    let secondParam = getParam(input, modes % 10, address + 2, relativeBase);
    modes /= 10;
    saveAtDestination(input,address+3,modes%10,relativeBase,0);
    if firstParam == secondParam {
        saveAtDestination(input,address+3,modes%10,relativeBase,1);
    }
}

fn opcode9(input: &mut [i64], address: usize, code: i64, relativeBase: i64) -> i64 {
    let mut modes = code;
    let mut base = input[address + 1];
    if modes % 10 == 0 {
        base = input[base as usize];
    }
    if modes % 10 == 2 {
        base = input[(base + relativeBase) as usize];
    }
    return base;
}

fn getCopy(input: &[i64]) -> Vec<i64> {
    let mut copy = vec![0; input.len()];
    copy.clone_from_slice(input);
    return copy;
}


fn main() {
    let input = [1102, 34463338, 34463338, 63, 1007, 63, 34463338, 63, 1005, 63, 53, 1101, 3, 0, 1000, 109, 988, 209, 12, 9, 1000, 209, 6, 209, 3, 203, 0, 1008, 1000, 1, 63, 1005, 63, 65, 1008, 1000, 2, 63, 1005, 63, 904, 1008, 1000, 0, 63, 1005, 63, 58, 4, 25, 104, 0, 99, 4, 0, 104, 0, 99, 4, 17, 104, 0, 99, 0, 0, 1101, 27, 0, 1014, 1101, 286, 0, 1023, 1102, 1, 35, 1018, 1102, 20, 1, 1000, 1101, 26, 0, 1010, 1101, 0, 289, 1022, 1102, 1, 30, 1019, 1102, 734, 1, 1025, 1102, 1, 31, 1012, 1101, 25, 0, 1001, 1102, 1, 1, 1021, 1101, 0, 36, 1002, 1101, 0, 527, 1028, 1101, 895, 0, 1026, 1102, 1, 23, 1016, 1101, 21, 0, 1003, 1102, 22, 1, 1011, 1102, 1, 522, 1029, 1102, 1, 892, 1027, 1102, 1, 0, 1020, 1102, 1, 28, 1015, 1102, 38, 1, 1006, 1101, 0, 32, 1008, 1101, 743, 0, 1024, 1101, 0, 37, 1007, 1102, 1, 24, 1013, 1102, 1, 33, 1009, 1102, 39, 1, 1004, 1102, 1, 34, 1005, 1102, 1, 29, 1017, 109, 19, 21102, 40, 1, -3, 1008, 1016, 40, 63, 1005, 63, 203, 4, 187, 1106, 0, 207, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -7, 2101, 0, -7, 63, 1008, 63, 32, 63, 1005, 63, 227, 1106, 0, 233, 4, 213, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -3, 2108, 37, -2, 63, 1005, 63, 255, 4, 239, 1001, 64, 1, 64, 1105, 1, 255, 1002, 64, 2, 64, 109, 11, 21108, 41, 40, -6, 1005, 1014, 275, 1001, 64, 1, 64, 1106, 0, 277, 4, 261, 1002, 64, 2, 64, 109, 10, 2105, 1, -7, 1105, 1, 295, 4, 283, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -27, 1201, -2, 0, 63, 1008, 63, 25, 63, 1005, 63, 321, 4, 301, 1001, 64, 1, 64, 1105, 1, 321, 1002, 64, 2, 64, 109, 15, 21107, 42, 41, 0, 1005, 1018, 341, 1001, 64, 1, 64, 1106, 0, 343, 4, 327, 1002, 64, 2, 64, 109, -25, 2108, 20, 10, 63, 1005, 63, 359, 1105, 1, 365, 4, 349, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 12, 2107, 35, 0, 63, 1005, 63, 385, 1001, 64, 1, 64, 1106, 0, 387, 4, 371, 1002, 64, 2, 64, 109, 4, 21101, 43, 0, 6, 1008, 1015, 43, 63, 1005, 63, 409, 4, 393, 1106, 0, 413, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 9, 21101, 44, 0, -8, 1008, 1010, 46, 63, 1005, 63, 437, 1001, 64, 1, 64, 1106, 0, 439, 4, 419, 1002, 64, 2, 64, 109, 5, 21108, 45, 45, -4, 1005, 1019, 457, 4, 445, 1106, 0, 461, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -22, 2102, 1, 7, 63, 1008, 63, 33, 63, 1005, 63, 481, 1106, 0, 487, 4, 467, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 14, 21102, 46, 1, -1, 1008, 1014, 43, 63, 1005, 63, 507, 1106, 0, 513, 4, 493, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 12, 2106, 0, 1, 4, 519, 1106, 0, 531, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -17, 1205, 10, 547, 1001, 64, 1, 64, 1106, 0, 549, 4, 537, 1002, 64, 2, 64, 109, -8, 1202, -2, 1, 63, 1008, 63, 17, 63, 1005, 63, 569, 1105, 1, 575, 4, 555, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 23, 1206, -5, 593, 4, 581, 1001, 64, 1, 64, 1105, 1, 593, 1002, 64, 2, 64, 109, -14, 1208, -8, 24, 63, 1005, 63, 613, 1001, 64, 1, 64, 1105, 1, 615, 4, 599, 1002, 64, 2, 64, 109, -2, 1207, -1, 33, 63, 1005, 63, 633, 4, 621, 1105, 1, 637, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 2, 21107, 47, 48, 5, 1005, 1016, 659, 4, 643, 1001, 64, 1, 64, 1105, 1, 659, 1002, 64, 2, 64, 109, -11, 1208, 8, 32, 63, 1005, 63, 681, 4, 665, 1001, 64, 1, 64, 1106, 0, 681, 1002, 64, 2, 64, 109, 2, 2101, 0, 0, 63, 1008, 63, 36, 63, 1005, 63, 703, 4, 687, 1106, 0, 707, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 12, 1206, 7, 719, 1106, 0, 725, 4, 713, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 2, 2105, 1, 8, 4, 731, 1001, 64, 1, 64, 1106, 0, 743, 1002, 64, 2, 64, 109, -21, 2102, 1, 9, 63, 1008, 63, 39, 63, 1005, 63, 769, 4, 749, 1001, 64, 1, 64, 1105, 1, 769, 1002, 64, 2, 64, 109, 11, 1201, -3, 0, 63, 1008, 63, 24, 63, 1005, 63, 793, 1001, 64, 1, 64, 1105, 1, 795, 4, 775, 1002, 64, 2, 64, 109, 20, 1205, -5, 809, 4, 801, 1105, 1, 813, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, -23, 1207, 4, 36, 63, 1005, 63, 833, 1001, 64, 1, 64, 1105, 1, 835, 4, 819, 1002, 64, 2, 64, 109, -3, 2107, 33, 5, 63, 1005, 63, 853, 4, 841, 1106, 0, 857, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 16, 1202, -9, 1, 63, 1008, 63, 37, 63, 1005, 63, 879, 4, 863, 1105, 1, 883, 1001, 64, 1, 64, 1002, 64, 2, 64, 109, 12, 2106, 0, -1, 1105, 1, 901, 4, 889, 1001, 64, 1, 64, 4, 64, 99, 21101, 0, 27, 1, 21101, 0, 915, 0, 1106, 0, 922, 21201, 1, 48476, 1, 204, 1, 99, 109, 3, 1207, -2, 3, 63, 1005, 63, 964, 21201, -2, -1, 1, 21101, 0, 942, 0, 1105, 1, 922, 21202, 1, 1, -1, 21201, -2, -3, 1, 21101, 0, 957, 0, 1105, 1, 922, 22201, 1, -1, -2, 1106, 0, 968, 21202, -2, 1, -2, 109, -3, 2106, 0, 0];
    let mut inputCopy = getCopy(&input); // for part 1
    inputCopy.resize(10000, 0);
    let mut diagnosticCode = intcodeCompute(&mut inputCopy, 1);
    println!("diagnostic code from part 1 {}", diagnosticCode);
    diagnosticCode = intcodeCompute(&mut inputCopy, 2);
    println!("diagnostic code from part 1 {}", diagnosticCode);
}
