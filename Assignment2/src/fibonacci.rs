pub fn fibo_series()
 {
   let mut num1=0;
   let mut num2=1;
   let mut fibo=0;
   let mut loopvar=0;
   print!("0");
   while loopvar < 10
    {
       fibo=num1+num2;
       print!(" {}",fibo);
       num1=num2;
       num2=fibo;
        loopvar +=1;
    }
 }