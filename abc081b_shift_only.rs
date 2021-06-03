fn main() {  
  let mut N = String::new();  
  std::io::stdin().read_line(&mut N).ok();  
  let N: usize = N.trim().parse().expect("Err");  
    
  let mut A = String::new();  
  std::io::stdin().read_line(&mut A).ok();  
  let mut A: Vec<&str> = A.split_whitespace().collect(); 
  let mut A_int: Vec<usize> = Vec::with_capacity(N); 
  for i in A { 
    A_int.push(i.trim().parse().expect("Err")); 
  } 
    
  let mut divine_flag = 1;  
  let mut divine_counter = 0;  
  let mut purpose_counter = 0;  
  while divine_flag==1 {  
    for i in 0..N {  
      //let a: usize = A[i].parse().expect("Err");  
      if A_int[i]%2==0 {  
        A_int[i] /= 2;  
        divine_counter += 1;  
      } else {  
        divine_flag = 0;  
        break;  
      }  
      if divine_counter==N {  
        divine_flag = 1;  
        purpose_counter += 1;  
      } else {  
        divine_flag = 0;  
      }  
    }  
    divine_counter = 0;  
  }  
  println!("{}", purpose_counter); 
}
