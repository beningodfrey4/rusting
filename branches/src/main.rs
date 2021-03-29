fn main() {
    let number = 3;
    if number < 5 {
        println!("condition is true");
    }
    else {
        println!("condition is false");
    }
    //if number {} will not work as no implicit type conversion to bool
    let tr = true;
    let mut _x = if tr {5} else {6};
    let mut arr:[u32; 5] = [1, 2, 3, 4, 5];
    arr[1] = 3;
    
    let mut sum = 0;
    let mut counter = 0;
    let mut _avg = loop {
        sum += arr[counter];
        if counter == 4 {
            break sum / 5;
        }
        else {
            counter += 1;
        }
    };
    
    counter = 0;
    while counter < 5 {
        sum += arr[counter];
        counter += 1;
    }

    for i in arr.iter() {
        sum += i;
    }
    
    for i in (1..11).rev() {    //similar to range in python
        println!("{} ", i);
    }

    
}
