use rand::Rng;


pub fn sort_iterative(x: &mut [u32]) {
    let n = x.len();
    let mut y = x.to_vec();
    let mut len = 1;

    while len < n {
        let mut i = 0;
        while i < n {
            if i + len >= n {
                y[i..].copy_from_slice(&x[i..]);
            } else if i + 2 * len > n {
                merge(&x[i..i+len], &x[i+len..], &mut y[i..]);
            } else {
                merge(&x[i..i+len], &x[i+len..i+2*len], &mut y[i..i+2*len]);
            }
            i += 2 * len;
        }
        len *= 2;
        x.copy_from_slice(&y);
        if len >= n {
            return;
        }
    }
}

pub fn sort_recursive(input: &mut [u32]) {
    let n = input.len();
    let half = n/2;
    let mut a = vec![0;half];
    let mut b =  vec![0;n-half];
    a.copy_from_slice(&input[..half]);
    b.copy_from_slice(&input[half..]);

    if half > 1 {
        sort_recursive(&mut a);
        sort_recursive(&mut b);
    }

    merge(&a,& b,input);
}



fn merge(a: &[u32], b: &[u32], out: &mut [u32]) {

    if out.len() != a.len() + b.len() {
        panic!("target has wrong size, cannot continue")
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < a.len() && j < b.len() {
        if a[i] > b[j] {
            out[k] = b[j];
            j += 1;
            k += 1;
        }
        else if a[i] < b[j] {
            out[k] = a[i];
            i += 1;
            k += 1;
        }
        else {  // equality, copy both
            out[k] = a[i];
            i += 1;
            k += 1;

            out[k] = b[j];
            j += 1;
            k += 1;
        }
    }

    // perhaps there is something left

    if i < a.len() {
        out[k..].copy_from_slice(&a[i..]);
    }

    if j < b.len() {
        out[k..].copy_from_slice(&b[j..]);

    }


}



pub fn generate_data() -> Vec<u32> {
    generate_custom_data(20)
}

pub fn generate_custom_data(exponent: u32) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut data = vec![0; 2usize.pow(exponent)];
    //let mut data = vec![0; (2usize.pow(exponent) as f32 * 1.1f32) as usize];
    for i in &mut data {
        *i = rng.gen();
    }
    data
}

pub fn check_sorted(data: &[u32]) {
    let mut last = 0;
    for i in data {
        if last > *i {
            panic!("not sorted correctly");
        }
        last = *i;
    }
}
