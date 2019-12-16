extern crate num;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::time::Instant;
use num::integer::lcm;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct vector {
    x: i32,
    y: i32,
    z: i32,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Moon {
    position: vector,
    velocity: vector,
    init_position: vector,
}

impl Moon {
    fn step(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }

    fn simulate(&mut self, other_moon: &Moon) {
        if self.position.x < other_moon.position.x {
            self.velocity.x += 1;
        }
        if self.position.x > other_moon.position.x {
            self.velocity.x -= 1;
        }

        if self.position.y < other_moon.position.y {
            self.velocity.y += 1;
        }
        if self.position.y > other_moon.position.y {
            self.velocity.y -= 1;
        }

        if self.position.z < other_moon.position.z {
            self.velocity.z += 1;
        }
        if self.position.z > other_moon.position.z {
            self.velocity.z -= 1;
        }
    }

    fn energy(&self) -> i32 {
        return (self.position.x.abs() + self.position.y.abs() + self.position.z.abs()) * (self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs());
    }

    fn isX(&self) -> bool {
        return self.position.x == self.init_position.x && self.velocity.x == 0;
    }
    fn isY(&self) -> bool {
        return self.position.y == self.init_position.y && self.velocity.y == 0;
    }
    fn isZ(&self) -> bool {
        return self.position.z == self.init_position.z && self.velocity.z == 0;
    }
}


fn simulate(moons: &mut Vec<Moon>) {
    let length = moons.len();
    for i in 0..length {
        let mut first_moon = moons[i];
        for j in 0..length {
            let second_moon = moons[j];
            first_moon.simulate(&second_moon);
        }
        moons[i] = first_moon;
    }
}

fn doStep(moons:&mut Vec<Moon>){
    for i in 0..moons.len(){
        let mut moon = moons[i];
        moon.step();
        moons[i] = moon;
    }
}

fn isX(moons:&mut Vec<Moon>) -> bool{
    let mut state = true;
    for i in 0..moons.len(){
        let moon = moons[i];
        state = state && moon.isX();
    }
    return state;
}

fn isY(moons:&mut Vec<Moon>) -> bool{
    let mut state = true;
    for i in 0..moons.len(){
        let moon = moons[i];
        state = state && moon.isY();
    }
    return state;
}

fn isZ(moons:&mut Vec<Moon>) -> bool{
    let mut state = true;
    for i in 0..moons.len(){
        let moon = moons[i];
        state = state && moon.isZ();
    }
    return state;
}

fn getEnergy(moons:&Vec<Moon>)->i32{
    let mut energy = 0;
    for moon in moons{
        energy += moon.energy()
    }
    energy
}


fn main() {
    let mut energy = 0;
    let mut x = 0;
    let mut y = 0;
    let mut z = 0;
    let mut defaultVelocity = vector { x: 0, y: 0, z: 0 };
    let moon1 = Moon { position: vector { x: 5, y: 4, z: 4 }, velocity: defaultVelocity, init_position: vector { x: 5, y: 4, z: 4 } };
    let moon2 = Moon { position: vector { x: -11, y: -11, z: -3 }, velocity: defaultVelocity, init_position: vector { x: -11, y: -11, z: -3 } };
    let moon3 = Moon { position: vector { x: 0, y: 7, z: 0 }, velocity: defaultVelocity, init_position: vector { x: 0, y: 7, z: 0 } };
    let moon4 = Moon { position: vector { x: -13, y: 2, z: 10 }, velocity: defaultVelocity, init_position: vector { x: -13, y: 2, z: 10 } };
    let mut moons = vec![moon1, moon2, moon3, moon4];
    let mut i: i64 = 0;
    let start = Instant::now();
    loop {
        simulate(&mut moons);
        doStep(&mut moons);
        i += 1;
        if i == 1000{
            energy = getEnergy(&moons);
        }
        if isY(&mut moons) && y==0 {
            y = i;
        }
        if isX(&mut moons) && x==0{
            x = i;
        }
        if isZ(&mut moons) && z == 0{
            z = i;
        }
        if x > 0 && y > 0 && z > 0{
            break;
        }
    }
    let duration = start.elapsed();
    let mut result = lcm(x,y);
    result = lcm(result,z);
    println!("part 1 answer is {}",energy);
    println!("part 2 answer is {}",result);
    println!("{:?}", duration);
}
