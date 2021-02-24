mod binary_linear_search_ques;
mod leap_year_count;
mod merge_sort_program;

fn main() {
    let arr =[10,20,30,50,70,90];
    let first = 0;
    let last = arr.len()-1;
    let num = 50;
    let index = 0;

    binary_linear_search_ques::linear_search(&arr, num, index);
    println!(" ");
    binary_linear_search_ques::binary_search(&arr,first,last,num);
    println!(" ");

    ///Taking Array of tuples as input in (dd,mm,year)
    let date = [(01,02,2020),(03,04,2021),(13,09,2024)];
    let index=0;
    let count=0;
    println!(" ");
    leap_year_count::leap_year_counter(&date,index,count);

    ///defining a new array for merge sort
    let mut array = [12, 11, 13, 5, 6, 7];
    println!(" ");
    println!("Merge Sort");
    println!("Before Sorting");
    println!("array is {:?}",array);
    let len = array.len() - 1;
    merge_sort_program::sort(&mut array, 0,len);
    println!("-----------------------");
    println!("After Sorting");
    println!("array is {:?}",array);

}
