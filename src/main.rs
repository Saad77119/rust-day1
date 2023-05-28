use std::io;
fn main() {

    let mut input = String::new();
    take_distance(&mut input);
    println!("distance from {} ",input);
     calculate_distance_from_world(&mut input);
    println!("distance from {} ",input);
    println!("distance from {} ",&input);
    println!("distance from {} ",&mut input);
}


    fn calculate_distance_from_world( distance :&mut String) -> String {
        println!("Number1: {}", distance);
       
        let parsed_distance: i64 = distance.trim().parse().expect("Failed to calculate distance from input");
    
        println!("number {} ",parsed_distance);
        let calculated_distance = (parsed_distance * 100).to_string();
        distance.clear();
        distance.push_str(&calculated_distance);
      
        return calculated_distance;
        
    }
fn take_distance(distance_input : &mut String) {
    io::stdin().read_line(distance_input).expect("Failed to read distance from input");

}
