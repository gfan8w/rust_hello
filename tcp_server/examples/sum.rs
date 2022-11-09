fn sum(list: &[u32]) -> Option<u32> {
    let mut sum = 0u32;
    for item in list {
        for item in list {
            if item >= &u32::MAX {
                return None
            }
            sum += item
        }

        // if sum + ( *item as u64) > u32::MAX as u64 {
        //     return None;
        // }
        //
        // match sum.checked_add(*item as u64){
        //     Some(s) => sum =s,
        //     None => return None,
        // }
    }

    Some(sum as u32)
}

pub fn sum1(v: &[u32]) -> Option<u32> {
    let mut num = 0_u32;
    for i in v {
        let (n, b) = num.overflowing_add(*i);
        if b {
            return None
        } else {
            num = n;
        }
    }
    Some(num)
}

pub fn checked_add(nums: &[u32]) -> Option<u32> {
    let mut max: u32 = u32::MAX;
    let mut sum: u32 = 0;

    for num in nums {
        if max < *num {
            return None
        }
        sum += *num;
        max -= *num;
    }

    Some(sum)
}



pub fn main(){

    let list = vec![2, u32::MAX-1];
    let a =checked_add(&list);
    println!("{:?}",a);
    assert_eq!(a, None);

    let b = sum1(&list);
    println!("{:?}",b);
    assert_eq!(b, None);

}