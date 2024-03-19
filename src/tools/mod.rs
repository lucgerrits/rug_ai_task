use std::env;

pub fn load_and_check_env() {
    dotenv::dotenv().ok();
    match env::var("BASE_DATA_URL") {
        Ok(_) => {}
        Err(_) => panic!("BASE_DATA_URL must be set"),
    };
    match env::var("LOG_LEVEL") {
        Ok(_) => {}
        Err(_) => env::set_var("LOG_LEVEL", "info"),
    }

    match env::var("MAX_THREADS") {
        Ok(_) => {}
        Err(_) => {
            let num_cpu = num_cpus::get();
            env::set_var("MAX_THREADS", &num_cpu.to_string()) // default to the number of CPUs
        }
    }

    match env::var("CACHE_MERGED_DATA") {
        Ok(_) => {}
        Err(_) => env::set_var("CACHE_MERGED_DATA", "true"),
    }
    
    env_logger::Builder::from_env("LOG_LEVEL").init();
}
