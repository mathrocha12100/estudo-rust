pub fn references_rs() {
     let mut x = 10;
     let y = &mut x;

     *y += 1;
     
     println!("y {}", y);
}