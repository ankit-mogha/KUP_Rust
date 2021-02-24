mod binary_linear_search_ques;
mod leap_year_count;
//mod merge_sort_ques;

fn main() {
    let arr =[10,20,30,50,70,90];
    let first = 0;
    let last = arr.len()-1;
    let num = 50;
    let index = 0;

    binary_linear_search_ques::linear_search(&arr, num, index);
    binary_linear_search_ques::binary_search(&arr,first,last,num);

    let date = [(01,02,2020),(03,04,2021),(13,09,2024)]; ///Taking Array of tuples as input in (dd,mm,year)
    let index=0;
    let count=0;
    leap_year_count::leap_year_counter(&date,index,count);
}
