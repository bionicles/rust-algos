// Skyline Heights problem, Algorithm Challenges pg. 40

fn main() {
    let skyline = vec![-1, 1, 1, 7, 3];
   let mut res = vec![];
    println!("initial result: {:?}", res);
    for (i, building) in skyline.iter().enumerate() {
        println!("{} {}", i, building);
        if building <= &0 {
            println!("invisible!");
            println!("");
            continue
        }
        let prev_buildings = &skyline[0..i];
        println!("prev: {:?}", prev_buildings);
        let tallest = prev_buildings.iter().max();
        match tallest {
            None => println!("bion sucks at rust!"),
            Some(_i32) => {
                println!("tallest: {}", tallest.unwrap());
                if building > tallest.unwrap() {
                    let building_copy: i32 = *building;
                    res.push(building_copy)
                }
            }
        }
        println!("")
    }
    println!("we can see:");
    println!("{:?}", res);
    println!("expected:");
    println!("[1, 7]");
}