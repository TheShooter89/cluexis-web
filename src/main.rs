mod globals;

fn app_info() {
    println!("#################################################################");
    println!("application host: {:?}", globals::HOST);
    println!("application port: {:?}", globals::PORT);
    println!("application data folder: {:?}", globals::APP_DATA_ROOT);
    println!(
        "application database folder: {:?}",
        globals::DATABASE_FOLDER
    );
    println!("application database name: {:?}", globals::DATABASE_NAME);
    println!("application database path: {:?}", globals::DATABASE_PATH);
    println!("#################################################################");
}

fn main() {
    println!("Hello, develop!\n");
    app_info();
}
