const STARTING_AMOUNT :i32 = 6;
const READY_AMOUNT : i32 = 2;
    
fn main() {  
    let (mut missiles, ready) : (i32, i32) = (STARTING_AMOUNT, READY_AMOUNT);
    
    println!("Firing {} of {} my missiles...", ready, missiles);
    missiles -= ready;
    println!("{} missiles left.", missiles);

}
