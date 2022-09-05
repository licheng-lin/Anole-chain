- [ ] ToDoList 
    - [X] complete simchain-server
    - [X] format schnorr
    - [X] complete signature use schnorr
    - [ ] query verify


Input format

```
block_id [address] {in/out, amount, timestamp}
```

如何对交易进行签名？目前RawTransaction 的结构体如下。

1. 将所有数据进行字符串拼接进行签名
2. 将数据结构转为Serializable，然后对字符串进行签名

```
pub struct RawTransaction {
    pub block_id: IdType,
    pub key: KeyType,
    pub value: TransactionValue,
}
```

经讨论确定以第一种方式进行签名。

```index
fn lr(arr_x: &[f32], arr_y: &[f32]) -> (f32, f32) {      //linear regression
    let mut sum_x: f32 = 0.0;
    let mut sum_y: f32 = 0.0;
    let mut sum_a1: f32 = 0.0;
    let mut sum_a2: f32 = 0.0;
    let len_x = arr_x.len();
    for k in 0..len_x {
        sum_x = sum_x + arr_x[k];
        sum_y = sum_y + arr_y[k];
    }
    let len_y: f32 = len_x as f32;
    let avg_x: f32 = sum_x / len_y;
    let avg_y: f32 = sum_y / len_y;
    for k in 0..len_x {
        sum_a1 = sum_a1 + (arr_x[k] - avg_x) * (arr_y[k] - avg_y);
        sum_a2 = sum_a2 + (arr_x[k] - avg_x) * (arr_x[k] - avg_x);
    }
    let r_a: f32 = sum_a1 / sum_a2;
    let r_b: f32 = avg_y - r_a * avg_x;
    (r_a, r_b)
}

fn vbs(arr_px: &[f32], t: f32) -> f32 {     // variant binary search
    let len_px = arr_px.len();
    let mut low = 0;
    let mut high = len_px - 1;
    while low <= high {
        let mid = (low + high)/2;
        if arr_px[mid] >= t {
            if mid == 0 || arr_px[mid+1] < t {
                return arr_px[mid]; 
            } else {
                high = mid - 1;
            }
        } else {
            low = mid + 1;
        }
    }
    return arr_px[low-1];
}


fn main() {  // piecewise linear regression
    let x = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.0, 19.0,
    20.0, 21.0];
    let y = [1.0, 2.0, 2.0, 3.0, 10.0, 12.0, 15.0, 16.0, 11.0, 12.0, 13.0, 25.0, 27.0, 28.0, 30.0, 32.0, 24.0,
    25.0, 27.0, 30.0, 21.0];   //test data

    let max_err = 1.0;
    let min_err = -1.0;
    let len_arx = x.len();
    let (a0, b0) = lr(&x[0..2], &y[0..2]);
    let mut v_a = vec![a0];   //The set of slopes a
    let mut v_b = vec![b0];   //The set of intercepts b
    let mut px = vec![x[0]];  //The set of segment points px
    let mut p_x = 0;
    for i in 2..len_arx {
        let mut flag = 0;
        let a_len = v_a.len();
        let b_len = v_b.len();
        if (y[i] - (v_a[a_len - 1] * x[i] + v_b[b_len - 1])) < min_err || (y[i] - (v_a[a_len - 1] * x[i] + v_b[b_len - 1])) > max_err {
            let (a, b) = lr(&x[p_x..i+1], &y[p_x..i+1]);   // The current value exceeds the threshold, regression again.
            for j in p_x..i+1 {   //Check the regression
                if (y[j] - (a * x[j] + b)) < min_err || (y[j] - (a * x[j] + b)) > max_err {
                    px.push(x[i-1]);  //There is a value that exceeds the threshold, and x[i-1] is the segment point
                    flag = 1;
                    p_x = i - 1;
                    break;
                }
            }
            if flag == 0 {  //No value exceeds the threshold and the regression results are updated
                v_a[a_len-1] = a;
                v_b[b_len-1] = b;
            }
        }
        if flag == 1 && i < len_arx {   //There is a new segment, new linear regression
            let (a1, b1) = lr (&x[i-1..i+1], &y[i-1..i+1]);
            v_a.push(a1);
            v_b.push(b1);
        }
    }
    px.push(x[len_arx-1]);   //The last point

    println!("The value of px is: {:?}", px);
    println!("The value of A is: {:?}", v_a);
    println!("The value of B is: {:?}", v_b);
    println!("len_arx:{}", len_arx);
  
    let pt = vbs(&px, 0.0);
    println!("The value of pt is: {:?}", pt);

}
```
