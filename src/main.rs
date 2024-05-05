use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Usage: nub_manager <command>");
        return;
    }
    
    let command = &args[1];
    
    match command.as_str() {
        "install" => install_command(&args),
        "create" => create_command(&args),
        "add" => add_command(&args),
        "remove" => remove_command(&args),
        "delete" => delete_command(&args),
        "--help" => display_help(),
        _ => println!("Usage: nub_manager <command>")
    }
}

// Команда встановлення
fn install_command(args: &[String]) {
    match args.get(2) {
        Some(installed_object) => match installed_object.as_str() {
            "niklang" => install_niklang(),
            "udav" => install_udav(),
            "based" => install_based(),
            "niklang-ext" => install_niklang_ext(),
            "udav-ext" => install_udav_ext(),
            "based-ext" => install_based_ext(),
            "nub-ide" => install_nub_ide(),
            _ => println!("Unknown installed object: {}", installed_object),
        },
        None => println!("Usage: nub_manager install <installed_object>"),
    }
}

// Команда створення пустого проєкту
fn create_command(args: &[String]) {
    match (args.get(2), args.get(3)) {
        (Some(type_project), Some(name_project)) => match type_project.as_str() {
            "niklang-project" => create_niklang_project(name_project),
            "udav-project" => create_udav_project(name_project),
            "based-project" => create_based_project(name_project),
            _ => println!("Unknown project type: {}", type_project),
        },
        _ => println!("Usage: nub_manager create <type_project> <name_project>"),
    }
}

// Команда додавання бібліотеки
fn add_command(args: &[String]) {
    match (args.get(2), args.get(3)) {
        (Some(type_lib), Some(name_lib)) => match type_lib.as_str() {
            "niklang-lib" => add_niklang_lib(name_lib),
            "udav-lib" => add_udav_lib(name_lib),
            "based-lib" => add_based_lib(name_lib),
            _ => println!("Unknown library type: {}", type_lib),
        },
        _ => println!("Usage: nub_manager add <type_lib> <name_lib>"),
    }
}

// Команда вилучення бібліотеки
fn remove_command(args: &[String]) {
    match (args.get(2), args.get(3)) {
        (Some(type_lib), Some(name_lib)) => match type_lib.as_str() {
            "niklang-lib" => remove_niklang_lib(name_lib),
            "udav-lib" => remove_udav_lib(name_lib),
            "based-lib" => remove_based_lib(name_lib),
            _ => println!("Unknown library type: {}", type_lib),
        },
        _ => println!("Usage: nub_manager remove <type_lib> <name_lib>"),
    }
}

// Команда видалення
fn delete_command(args: &[String]) {
    match args.get(2) {
        Some(deleted_object) => match deleted_object.as_str() {
            "niklang" => delete_niklang(),
            "udav" => delete_udav(),
            "based" => delete_based(),
            "niklang-ext" => delete_niklang_ext(),
            "udav-ext" => delete_udav_ext(),
            "based-ext" => delete_based_ext(),
            "nub-ide" => delete_nub_ide(),
            _ => println!("Unknown deleted object: {}", deleted_object),
        },
        None => println!("Usage: nub_manager delete <deleted_object>"),
    }
}

// Встановлення NikLang
fn install_niklang() {
    println!("Install NikLang");
}

// Встановлення Udav
fn install_udav() {
    println!("Install Udav");
}

// Встановлення Based
fn install_based() {
    println!("Install Based");
}

// Встановлення розширення NikLang для VS code
fn install_niklang_ext() {
    println!("Install VS code ext for NikLang");
}

// Встановлення розширення Udav для VS code
fn install_udav_ext() {
    println!("Install VS code ext for NikLang");
}

// Встановлення розширення Based для VS code
fn install_based_ext() {
    println!("Install VS code ext for NikLang");
}

// Встановлення NUB IDE
fn install_nub_ide() {
    println!("Install NUB IDE");
}

// Створення пустого NikLang проєкту
fn create_niklang_project(project_name: &str) {
    println!("Create empty NikLang {} project", project_name);
}

// Створення пустого Udav проєкту
fn create_udav_project(project_name: &str) {
    println!("Create empty Udav {} project", project_name);
}

// Створення пустого Based проєкту
fn create_based_project(project_name: &str) {
    println!("Create empty Based {} project", project_name);
}

// Додання стандартної бібліотеки для NikLang
fn add_niklang_lib(lib_name: &str) {
    println!("Add {} lib in NikLang project", lib_name);
}

// Додання стандартної бібліотеки для Udav
fn add_udav_lib(lib_name: &str) {
    println!("Add {} lib in NikLang project", lib_name);
}

// Додання стандартної бібліотеки для Based
fn add_based_lib(lib_name: &str) {
    println!("Add {} lib in NikLang project", lib_name);
}

// Додання стандартної бібліотеки для NikLang
fn remove_niklang_lib(lib_name: &str) {
    println!("Remove {} lib in NikLang project", lib_name);
}

// Додання стандартної бібліотеки для Udav
fn remove_udav_lib(lib_name: &str) {
    println!("Remove {} lib in NikLang project", lib_name);
}

// Додання стандартної бібліотеки для Based
fn remove_based_lib(lib_name: &str) {
    println!("Remove {} lib in NikLang project", lib_name);
}

// Видалення NikLang
fn delete_niklang() {
    println!("Delete NikLang");
}

// Видалення Udav
fn delete_udav() {
    println!("Delete Udav");
}

// Видалення Based
fn delete_based() {
     println!("Delete Based");
}

// Видалення розширення NikLang для VS code
fn delete_niklang_ext() {
    println!("Delete VS code ext for NikLang");
}

// Видалення розширення Udav для VS code
fn delete_udav_ext() {
    println!("Delete VS code ext for NikLang");
}

// Видалення розширення Based для VS code
fn delete_based_ext() {
    println!("Delete VS code ext for NikLang");
}

// Видалення NUB IDE
fn delete_nub_ide() {
    println!("Delete NUB IDE");
}

// Показати довідку
fn display_help() {
    println!("About this manager");
}
