mod outside_module;

fn trait_generic_module_tests()
{
    use module::Person;
    
    pub mod module
    {
        pub mod module2
        {
            pub struct Woman
            {
                pub name:String
            }
    
            pub struct Man
            {
                pub name:String
            }
        }

        pub trait Person
        {
            fn get_name(&self) -> String;
        }

        impl Person for super::outside_module::m::Woman2
        {
            fn get_name(&self) -> String
            {
                return format!("Ms. {}, the outsider!", self.name.to_string());
            }
        }

        impl Person for super::outside_module::m::Man2
        {
            fn get_name(&self) -> String
            {
                return format!("Mr. {}, the outsider!", self.name.to_string());
            }
        }
        
        impl Person for module2::Woman
        {
            fn get_name(&self) -> String
            {
                return format!("Ms. {}", self.name.to_string());
            }
        }
    
        impl Person for module2::Man
        {
            fn get_name(&self) -> String
            {
                return format!("Mr. {}", self.name.to_string());
            }
        }
        
        pub fn show<T:Person>(p:T)
        {
            println!("From Generic: {}", p.get_name());
        }
    }


    let man = module::module2::Man{name:"Ronaldo".to_string()};
    let man2 = outside_module::m::Man2{name:"Ronaldo".to_string()};
    let woman = module::module2::Woman{name:"Rosana".to_string()};
    let woman2 = outside_module::m::Woman2{name:"Rosana".to_string()};
    println!("{}",man.get_name());
    println!("{}",woman.get_name());
    println!("{}",man2.get_name());
    println!("{}",woman2.get_name());

    module::show(man);
    module::show(woman);
    module::show(man2);
    module::show(woman2);
}

mod training_functions;


pub mod chapter {
    pub mod lesson {
        pub fn summary(){
            println!("Summary"); 
        } 
        pub mod heading {  
            pub fn illustration() {  
              println!("Content visualization");
            }
        }
    }
}


fn main()
{
    let run = false;
    if run
    {
        training_functions::hello_world_example();
        training_functions::var_tests();
        training_functions::array_tests();
        training_functions::tuple_test();
        training_functions::ptr_tests();
        training_functions::scrutnee_test();
        training_functions::match_test();
        training_functions::loop_test();

        training_functions::str_tests();
        training_functions::vector_tests();
        training_functions::enum_tests();
        training_functions::struct_tests();
        training_functions::option_results_tests();
        trait_generic_module_tests();
        training_functions::fn_param_tests();
    }
    
    println!("{}", unsafe { training_functions::cpp_bind::add(10,30) });
}
