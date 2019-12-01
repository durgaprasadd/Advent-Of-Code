fn get_amount_of_fuel_required(mass:i32)->i32{
    return (mass/3) - 2;
}

fn get_total_amount_of_fuel_required(input_masses: &[i32]) -> i32{
    let mut total =  0;
    for mass in input_masses{
        total += get_amount_of_fuel_required(*mass);
    }
    return total;
}

fn get_total_amount_of_fuel_required_including_fuel_mass(input_masses:&[i32]) -> i32{
    let mut total = 0;
    for mass in input_masses{
        let mut fuel_required = get_amount_of_fuel_required( *mass);
        while fuel_required > 0 {
            total += fuel_required;
            fuel_required = get_amount_of_fuel_required(fuel_required);
        }
    }
    return total;
}


fn main() {
    const INPUT:[i32;100] = [78207, 89869, 145449, 73634, 78681, 81375, 131482, 126998, 50801, 115839, 77949, 53203, 146099, 56912, 59925, 132631, 115087, 89543, 123234, 108110, 109873, 81923, 124264, 87981, 106554, 147239, 73615, 72609, 129684, 84175, 64915, 98124, 74391, 55211, 120961, 119116, 148275, 89605, 115986, 120547, 50299, 137922, 78906, 145216, 80424, 122610, 61408, 97573, 127533, 116820, 76068, 77400, 117943, 85231, 102442, 62002, 58761, 56479, 98200, 85971, 73985, 88908, 82719, 120604, 83378, 88241, 122574, 76731, 99810, 137548, 102617, 105352, 137585, 83238, 118817, 149419, 107629, 63893, 56049, 70693, 83844, 76413, 87021, 90259, 124289, 102527, 139625, 106607, 120241, 101098, 66142, 96591, 82277, 142297, 116671, 131881, 94861, 79741, 73561, 115214 ];

    // part 1
    let total_amount_of_fuel_required = get_total_amount_of_fuel_required(&INPUT);
    println!("total amount of fuel required : {}",total_amount_of_fuel_required);

    // part 2
    let total_amount_of_fuel_required_including_fuel_mass = get_total_amount_of_fuel_required_including_fuel_mass(&INPUT);
    println!("total amount of fuel required including fuel mass : {}",total_amount_of_fuel_required_including_fuel_mass);
}
