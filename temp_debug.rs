fn main() { 
    let naming = cli_frontend::naming::SmartNaming::new();
    println!("Input: user");
    println!("PascalCase: {}", naming.to_pascal_case("user"));
    let result = naming.process_smart_names("user");
    println!("Hook name: {}", result.hook_name);
}
