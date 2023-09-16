
fn main()
{
    cc::Build::new()
        .file("src/cpp/my_dll.cpp")
        .compile("my_dll");
}