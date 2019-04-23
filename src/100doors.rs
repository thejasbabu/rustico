// Problem statement: http://www.rosettacode.org/wiki/100_doors
fn main() {
    let mut doors = [0; 100];
    for pass in 1..=100 {
        doors = match pass {
            1 => open_door(),
            x => open_close_door(x, doors)
        }
    }
    println!("First: {}, Second: {}, Fourth: {} and Sixth: {}", doors[0], doors[1], doors[3], doors[5]);
}

fn open_close_door(pass:i32, mut doors: [i32; 100]) -> [i32; 100]{
    for i in pass - 1..doors.len() as i32 {
        doors[i as usize] = match (i + 1) % pass  {
             0 => negate(doors[i as usize]),
             _ => doors[i as usize],
        }
    }
    doors
}

fn negate(val:i32) -> i32 {
    match val {
        0 => 1,
        _ => 0,
    }
}

fn open_door() -> [i32; 100]{
    [1; 100]
}