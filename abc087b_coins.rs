fn main() { 
  let mut A = String::new(); 
  std::io::stdin().read_line(&mut A).ok(); 
  let A: usize = A.trim().parse().expect("Err"); 
  let mut B = String::new(); 
  std::io::stdin().read_line(&mut B).ok(); 
  let B: usize = B.trim().parse().expect("Err"); 
  let mut C = String::new(); 
  std::io::stdin().read_line(&mut C).ok(); 
  let C: usize = C.trim().parse().expect("Err"); 
  let mut X = String::new(); 
  std::io::stdin().read_line(&mut X).ok(); 
  let X: usize = X.trim().parse().expect("Err"); 
   
  let mut count = 0; 
  for a in 0..=A { 
    for b in 0..=B { 
      for c in 0..=C { 
        if X == 500*a+100*b+50*c { 
          count += 1; 
        } 
      } 
    } 
  } 
  println!("{}", count); 
}
