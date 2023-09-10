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



fn fn_param_tests()
{
    
    fn fn1(x:&mut i32, y: &mut i32) -> (i32, [i32;2])
    {
        let tmp: i32 = *x;
        *x = *y;
        *y = tmp;

        return (*x * *y, [*x, *y]);
    }

    let mut x = 5;
    let mut y = 2;
    println!("{} - {}", x, y);
    let tuple_return = fn1(&mut x, &mut y);
    println!("({} - {:?})", tuple_return.0, tuple_return.1);
    println!("{} - {}", x, y);
}

fn str_tests()
{
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

    //objs need to borrow again because it has been used before
    let str1 = "concat".to_string();
    let str2 = " string".to_string();
    let i = 10;
    let format_str = format!("{} {}{}", i, str1, str2);
    println!("{}", format_str);

    println!("{}", &my_str[0..7]);

    let another_string = String::from("aaa bbb ccc ddd");
    another_string.chars().nth(0);
    another_string.starts_with('c');
    another_string.contains("b c");
    another_string.ends_with('d');
}

fn vector_tests()
{
    
    let mut my_vect:Vec<i32> = vec![];
    my_vect.push(1);
    my_vect.push(2);
    my_vect.push(3);
    println!("{:?}", my_vect);
    let el = my_vect.get(1);
    println!("{:?}", el);
    
    for my_el in &mut my_vect
    {
        *my_el = *my_el * 10;
    }

    println!("{:?} {:?}", my_vect, &my_vect[2..]);
    
    match my_vect.get(10)
    {
        Some(x) => 
        {
            println!("{}", x)  
        }
        None =>
        {
            println!("out of bounds");
        }
    }

    while !my_vect.is_empty()
    {
        match my_vect.pop()
        {
            Some(x) => 
            {
                println!("popped {}", x)  
            }
            None =>
            {
                println!("out of bounds");
            }
        }
    }

    match my_vect.pop()
    {
        Some(x) => 
        {
            println!("popped {}", x)  
        }
        None =>
        {
            println!("out of bounds");
        }
    }
}

fn struct_tests()
{
    #[derive(Debug)]
    struct MyStruct
    {
        my_int:i32,
        my_str:String,
        my_opt_param:Option<String>,
    }

    impl MyStruct
    {
        fn to_string(&self) -> String
        {
            let opt_param = match &self.my_opt_param
            {
                Some(c)=> c.to_string(),
                None => "".to_string()
            };
            return format!("my_int:{}\tmy_str:'{}'\tmy_opt_param:'{}'", self.my_int, self.my_str, opt_param);
        }
    }

    let mut my_struct = MyStruct
    {
        my_int:11,
        my_str:"MyBigString".to_string(),
        my_opt_param:None
    };

    println!("{}", my_struct.to_string());
    my_struct.my_opt_param = Some(String::from("new str on opt param"));
    println!("{}", my_struct.to_string());
    println!("{:?}", my_struct);

    struct MyTupleStruct(i32,String);

    impl MyTupleStruct
    {
        fn to_string(&self) -> String
        {
            return format!("0:{}\t1:{}", self.0, self.1);
        }
    }

    let my_tuple_struct = MyTupleStruct(10,"MySuperTypleString".to_string());
    println!("{}", my_tuple_struct.to_string());
}

fn enum_tests()
{
    #[derive(Debug)]
    enum Direction
    {
        Up(String),Down(i32),Left(f32),Right(char)
    }

    impl Direction
    {
        fn get(&self) -> String
        {
            match self
            {
                Direction::Up(up) =>
                {
                    return up.to_string();
                }
                _ =>
                {
                    panic!("cant get string from {:?}", self);
                }
            }
        }
    }

    let up = Direction::Up("Up String".to_string());
    let down = Direction::Down(2);
    let left = Direction::Left(3.0);
    let right = Direction::Right('R');
    println!("\"{}\":{:?} {:?} {:?} {:?}", up.get(), up, down, left, right);
}

fn option_results_tests()
{
    fn is_option(some_opt:Option<i32>) -> Result<i32, String>
    {
        if some_opt.is_some()
        {
            return Ok(some_opt.unwrap());
        }
        else
        {
            return Err("error".to_string());
        }
    }
    
    let mut my_opt_var = Some(123);
    if my_opt_var.is_some()
    {
        println!("changing - {:?}", is_option(my_opt_var));
        my_opt_var = None;
    }

    if my_opt_var.is_none()
    {
        println!("changed to none {:?}", is_option(my_opt_var));
    }
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
        str_tests();
        vector_tests();
        enum_tests();
        struct_tests();
        option_results_tests();
    }

    struct Woman
    {
        name:String
    }

    struct Man
    {
        name:String
    }

    trait Person
    {
        fn get_name(&self) -> String;
    }

    impl Person for Woman
    {
        fn get_name(&self) -> String
        {
            return format!("Ms. {}", self.name.to_string());
        }
    }

    impl Person for Man
    {
        fn get_name(&self) -> String
        {
            return format!("Mr. {}", self.name.to_string());
        }
    }

    println!("{}",Man{name:"Ronaldo".to_string()}.get_name());
    println!("{}",Woman{name:"Rosana".to_string()}.get_name());

}
