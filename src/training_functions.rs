use ferris_says::say;
use std::io::{stdout, BufWriter};

pub fn hello_world_example()
{
    let stdout = stdout();
    let message = String::from("Hello fellow Rustaceans!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(message.as_bytes(), width, &mut writer).unwrap();

    println!("Hello workd!");
    eprintln!("Error Hello world!");
}

pub fn var_tests()
{
    let default_var = "default var";
    let mut mut_var = "mut var";
    println!("{} - {}", default_var, mut_var);
    mut_var = "mut var 2";
    println!("{} - {}", default_var, mut_var);

    let mut_var2 = mut_var;
    mut_var = "mut var 3";
    println!("{}", mut_var2);
    println!("{}", mut_var);

    let my_str: String = String::from("my string");
    let my_str2 = my_str.clone();
    println!("{}", my_str2);
    println!("{}", my_str);

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

pub fn array_tests()
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

pub fn tuple_test()
{
    let tuple_test = ("mystring", 1234, 2.2, [1,2,3,4]);

    println!("{:?}", tuple_test);
    println!("0:{} 3:{}", tuple_test.0, tuple_test.2);
}

pub fn ptr_tests()
{
    
    let my_var: i32 = 10;
    let my_ptr_var: &i32 = &my_var;
    println!("{} - {}", my_var, my_ptr_var);

    let mut my_var2 = 22;
    println!("{}", my_var2);
    my_var2  = 222;

    let mut my_ptr_var2 = &mut my_var2;    
    println!("{:p}", my_ptr_var2);
    *my_ptr_var2 = 2222;

    let my_ptr_to_ptr_var = &mut my_ptr_var2;
    println!("{:p} {:p} {}", my_ptr_to_ptr_var, *my_ptr_to_ptr_var, **my_ptr_to_ptr_var);
    **my_ptr_to_ptr_var = 22222;
    println!("{:p} {:p} {}", my_ptr_to_ptr_var, *my_ptr_to_ptr_var, **my_ptr_to_ptr_var);
}

pub fn scrutnee_test()
{
    let test_tuple = (1 , 1, "", 1.1);
    //if (let (1, 1, "", 1.1) = test_tuple)
    {
        println!("tuple {:?}", test_tuple);
    }
}

pub fn match_test()
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

pub fn loop_test()
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

pub fn fn_param_tests()
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

    fn lifecycle_goner(str1: String, str2: String)
    {
        println!("{} {}", str1, str2);
    }

    fn lifecycle_preserver_readonly<'a>(str1: &'a String, str2: &'a String)
    {
        println!("{} {}", str1, str2);
    }

    fn elided_lifecycle_preserver_readonly(str1: &String, str2: &String) //contracted way to preserve lifecycle
    {
        println!("{} {}", str1, str2);
    }

    fn lifecycle_preserver_rw<'a>(str1: &'a mut String, str2: &'a mut String)
    {
        println!("{} {}", str1, str2);
        str1.push_str(" altered");
        str2.push_str(" altered");
    }
    
    fn elided_lifecycle_preserver_rw(str1: &mut String, str2: &mut String) //contracted way to preserve lifecycle
    {
        println!("{} {}", str1, str2);
        str1.push_str(" altered");
        str2.push_str(" altered");
    }
    
    //fn elided_lifecycle_preserver_may_give_new_one<'a>(str1: &'a mut String, str2: &'a mut String) -> &'a mut String
    fn lifecycle_preserver_may_give_new_one<'a, 'b>(str1: &'a mut String, str2: &'b mut String) -> &'a mut String
    {
        if str1 == "1"
        { 
            str1.push('1');
            return str1;
        }
        else
        {
            str1.push_str(str2.as_str());
            return str1;
        }
    }

    let str1 = String::from("1");
    let str2 = String::from("33");
    
    lifecycle_goner(str1, str2); //inside this fn str1,str2 will be gone from this context
    //println!("{} {}", str1, str2); //uncomment this line to see
    
    //creating those back
    let mut str1 = String::from("1");
    let mut str2 = String::from("33");
    
    println!("{} {}", str1, str2);
    lifecycle_preserver_readonly(&str1, &str2); //can only read but still lives in this context
    elided_lifecycle_preserver_readonly(&str1, &str2); //can only read but still lives in this context
    lifecycle_preserver_rw(&mut str1, &mut str2); //can read/write but still lives in this context
    elided_lifecycle_preserver_rw(&mut str1, &mut str2); //can read/write but still lives in this context
    println!("{} {}", str1, str2);
    
    lifecycle_preserver_may_give_new_one(&mut str1, &mut str2); //str1 and str2 went to fn having different lifecycles
    println!("{} {}", str1, str2);
    let my_new_str1 = lifecycle_preserver_may_give_new_one(&mut str1, &mut str2);
    //println!("{}", str1);//uncomment this line to see concurrent borrow failure between str1 and my_new_str1
    println!("{}", my_new_str1);
    println!("{}", str1);//str1 and my_new_str1 are not concurrent anymore, since my_new_str1 is not used anymore after last line
    println!("{}", str2);
}

pub fn str_tests()
{
    let mut my_str = String::new();
    println!("{}", my_str,);

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

pub fn vector_tests()
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

pub fn struct_tests()
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

pub fn enum_tests()
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
    use Direction::*;
    let left = Left(3.0);
    let right = Right('R');
    println!("\"{}\":{:?} {:?} {:?} {:?}", up.get(), up, down, left, right);
}

pub fn option_results_tests()
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

pub mod cpp_bind
{
    use std::os::raw::*;
    extern "C" {
        pub fn add(
            a: c_int,
            b: c_int
        ) -> c_int;
    }
}