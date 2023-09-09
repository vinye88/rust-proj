use ferris_says::say; // from the previous step
use std::io::{stdout, BufWriter};

fn hello_world_example()
{
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("Hello workd!");
    eprintln!("Error Hello world!");
}

fn var_tests()
{
    let default_var = "default var";
    let mut mut_var = "mut var";
    println!("{} - {}", default_var, mut_var);
    mut_var = "mut var 2";
    println!("{} - {}", default_var, mut_var);

    let shadow = "default shadow";
    println!("{}", shadow);
    {
        let mut shadow = "mod shadow";
        println!("{}", shadow);
        shadow = "mod shadow 2";
        println!("{}", shadow);
    }
    println!("{}", shadow);

    let myint:i64 = 123;
    let myfloat:f64 = 2.222;

    println!("myint:{} myfloat:{}", myint, myfloat);
}

fn array_tests()
{
    let array:[i32;4] = [1,2,3,4];
    let array_sliced:&[i32] = &array[2..4]; //[x..y] x=start index y=end index + 1
    println!("array({})={:?} sliced={:?}", array.len(), array, array_sliced);

    let mut arr = [1,2,3,4,5];
    for a in &mut arr
    {
        *a *= *a;
        println!("{}", a);
    }
}

fn tuple_test()
{
    let tuple_test = ("mystring", 1234, 2.2, [1,2,3,4]);

    println!("{:?}", tuple_test);
    println!("0:{} 3:{}", tuple_test.0, tuple_test.2);
}

fn ptr_tests()
{
    
    let my_var: i32 = 10;
    let my_ptr_var: &i32 = &my_var;
    println!("{} - {}", my_var, my_ptr_var);

    let mut myVar2 = 22;
    println!("{}", myVar2);

    let mut myPtrVar2 = &mut myVar2;    
    println!("{:p}", myPtrVar2);

    let myPtrToPtrVar = &mut myPtrVar2;
    println!("{:p} {:p} {}", myPtrToPtrVar, *myPtrToPtrVar, **myPtrToPtrVar);
}

fn scrutnee_test()
{
    let test_tuple = (1 , 1, "", 1.1);
    //if (let (1, 1, "", 1.1) = test_tuple)
    {
        println!("tuple {:?}", test_tuple);
    }
}

fn match_test()
{
    let my_match = 20;
    match my_match
    {
        1 =>
        {
            println!("one");
        }

        20 =>
        {
            println!("twenty");
        }

        _ =>
        {
            println!("other value")
        }
    }
}

fn loop_test()
{
    for (i,var) in (7..15).enumerate()
    {
        println!("var[{}]:{}", i, var);
    }
    
    let mut count = 0;
    loop
    {
        count+=1;
        
        if count < 10
        {
            println!("{}", count);
        }
        else if count < 15
        {
            continue;
        }
        else if count < 20
        {
            println!("{}", count);            
        }
        else
        {
            break;
        }
    }
}


fn fn1(x:&mut i32, y: &mut i32) -> (i32, [i32;2])
{
    let tmp: i32 = *x;
    *x = *y;
    *y = tmp;

    return (*x * *y, [*x, *y]);
}

fn fn_param_tests()
{
    let mut x = 5;
    let mut y = 2;
    println!("{} - {}", x, y);
    let tuple_return = fn1(&mut x, &mut y);
    println!("({} - {:?})", tuple_return.0, tuple_return.1);
    println!("{} - {}", x, y);
}

fn main()
{
    let run = false;
    if run
    {
        hello_world_example();
        var_tests();
        array_tests();
        tuple_test();
        ptr_tests();
        scrutnee_test();
        match_test();
        loop_test();
        fn_param_tests();
    }
    
    let mut my_str = String::new();

    my_str = String::from("my new string");
    println!("{}", my_str,);
    for str in my_str.split(' ')
    {
        println!("{}", str);
    }
    println!("{:?}", my_str.chars());

    let str1 = "concat".to_string();
    let str2 = " string".to_string();

    println!("{}", str1 + &str2);

    //need to borrow again because it has been used before
    let str1 = "concat".to_string();
    let str2 = " string".to_string();

    let format_str = format!("{}{}", str1, str2);
    println!("{}", format_str);

    println!("{}", &my_str[0..7]);

}
