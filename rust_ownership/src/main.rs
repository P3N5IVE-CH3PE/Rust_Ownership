
// Safety is the absence of undefined behavior.

//fn read(y: bool){ 
//        if y { 
//                println!("y is true!");
//                }

//        }


// Unsafe version of the former due to moving the call to read before the definition of x
//fn read(y: bool){
//	if y {
//	 println!("y is true!");
//	}
//}
fn main() {
//	let x = true;
//	read(x); 
//	read(x);
//	let x = true;

 
	let a = Box::new([0; 1_000_000]);
	let b = a;

	for C in b () {

		println!("{C}");
	}


}




