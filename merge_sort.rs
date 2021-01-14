const MAX_DATA: usize = 10;

fn merge_sort(
    arr: &mut [i32],
    temp: &mut [i32],
    l: &mut usize,
    r: &mut usize)
{
    if l >= r { return; }

    let mut mid: usize = (*l + *r) / 2;
    let mut mid_tmp: usize = mid + 1;

    merge_sort(arr, temp, l, &mut mid);
    merge_sort(arr, temp, &mut mid_tmp, r);

    for i in *l..=mid {
        temp[i] = arr[i];
    }

    let mut j: usize = *r;
    
    for i in mid_tmp..=*r {
        temp[i] = arr[j];
        j -= 1;
    }

    let mut i: usize = *l;
    let mut j: usize = *r;

    for k in *l..=*r {
        if temp[i] <= temp[j] {
            arr[k] = temp[i];
            i += 1;
        } else {
            arr[k] = temp[j];
            j -= 1;
        }
    }
}

fn main() {
    let mut arr: [i32; MAX_DATA] = [3,6,1,7,2,0,4,5,9,8];
    let mut temp: [i32; 10] = [0; 10];
    
    for i in 0..MAX_DATA {
        print!("{} ", &arr[i]);
    }
    print!("\n");
    let mut l: usize = 0;
    let mut r: usize = MAX_DATA - 1;

    merge_sort(&mut arr, &mut temp, &mut l, &mut r);

    for i in 0..MAX_DATA {
        print!("{} ", &arr[i]);
    }
    print!("\n");
}
