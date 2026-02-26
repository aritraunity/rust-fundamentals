fn main() {

    //Decision Making Statements
    //Defining if expressions
    let number = 9;
    if number > 7 {
        println!("Greater");
    }else {
        println!("Lesser")
    }

    //if - else if - else
    let number = 6;
    if number% 2 == 0 {
        println!("Divisible by 2"); 
    }else if number % 3 == 0 {
        println!("Divisible by 3"); 
    }else if number % 4 == 0 {
        println!("Divisible by 4"); 
    }else {
        println!("Not divisible by 2, 3, 4")
    }

    //Looping
    // loop {
    //     println!("Infinitely Loop !")
    // }

    //Breaking from a Loop
    let mut x = 0;
    loop {
        x += 1;
        if x == 3 {
            println!("Exiting at {x} iterations");
            break;
        }
    }

    //Adding a loop label for dis-ambiguity
    // Print a 3 x 3 Matrix 
    let mut counter = (0, 0);
    'columns: loop {
        counter.0 += 1;
        'rows: loop {
            counter.1 += 1;
            println!("({},{})", counter.0, counter.1);
            if counter.1 >= 3 {
                counter.1 = 0;
                break;
            }
        }
        if counter.0 >= 3{
            break;
        }
    }

    //Looping using while, Entry 
    x = 0;
    while (x < 3){
        x += 1;
        println! ("Value of x {x}");
    }

    let ar = [1,2,3,4,5,6,7];

    //Looping through a collection using for
    for a in ar {
        println! ("Elment in ar {a}");
    }

    //Looping with for Range of  numbers
    for i in (0..4){
        println!("{i}");
    }
}


