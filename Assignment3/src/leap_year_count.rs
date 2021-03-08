///This Function counts the number of leap years
///#Arguments
///
/// Array of date tuple, index as a loop variable, counter variable that store number of leap year
///
/// #Return
///
/// prints the number of leap year

pub fn leap_year_counter(arr:&[(usize,usize,usize)],index:usize,mut count:usize)
{
    let len  = arr.len();
    if index<len
    {
        let temp = arr[index];
        if temp.2 % 4==0
        {
            count = count+1;
        }
        let index=index+1;
        leap_year_counter(&arr,index,count);
    }
    else {
        println!("The number of leap years : {}",count);
    }
}