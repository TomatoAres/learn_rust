fn bubble_sort_num(nums: &mut Vec<usize>) {
    for _i in 1..nums.len() {
        for i in 1..nums.len() {
            if nums[i-1] > nums[i] {
                nums.swap(i-1, i);
            }
        }
    }
}
fn bubble_sort_any<T:PartialOrd>(arr: &mut[T]) {
    let len = arr.len();
    if len < 2 {
        return ;
    }
    // 每次选出一个放到最末，上限递减
    for max_index in (0..len).rev() {
        // 顺序标记，有序跳出
        let mut sorted = true;
        // 从头检测排序
        for current in 0..max_index {
            // 不满足顺序交换，向上冒泡
            if arr[current] > arr[current + 1] {
                arr.swap(current, current + 1);
                sorted = false;
            }
        }
        if sorted {
            break;
        }
    }
}

fn main() {

    let mut list = vec![1, 34, 50, 200, 34, 51, 25, 100, 65];
    bubble_sort_num(&mut list);
    println!("{:?}  ", list);
 
    let mut list = vec!['D', 'e', 'A', 'C', 'a', 'W'];
    bubble_sort_any(&mut list);
    println!("{:?}  ", list);

    println!("complete")
}