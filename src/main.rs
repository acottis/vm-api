use actix_web::{web, App, HttpServer, Responder};
use serde::Deserialize;
use std::process::Command;

const IP_ADDR: &'static str = "0.0.0.0";
const PORT: &'static str = "8080";

/// Information used to create a new virtual machine in Hyper-V 
#[derive(Debug,Deserialize)]
struct VirtualMachine{
    hostname:   Option<String>,
    cpus:       Option<u8>,
}

/// This is our handler for creating new VM's in hyper-v
async fn new(vm: web::Json<VirtualMachine>) -> impl Responder {  
    format!("Creating VM with Hostname: {:?} and {:?} cores", vm.hostname, vm.cpus)
}

/// This our handler for viewing the status
async fn status(vm: web::Query<VirtualMachine>) -> impl Responder {
    match &vm.hostname{
        Some(hn) => {
            let out = Command::new("./hyperv.ps1")
                .arg(format!("{}", hn))
                .output().expect("Didnt get result");
            match out.status.success() {
                true => format!("{:?}",out),
                false => format!("{:?}",out),
            }
        },
        None => {
            format!("We Require the ?hostname=STRING query parameter")
        }
    }
}

/// This is the default response if random path is specified
async fn default_response() -> impl Responder {
    format!("API Endpoint can be found at /api/v{{VersionNumber}}/...")
}

/// Main entry point that starts the web server and publishes our services/routes
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

HttpServer::new(|| 
    App::new().service(
        web::scope("/api/v0") 
            .route("new", web::post().to(new))
            .route("status", web::get().to(status))
        )
        .default_service(web::get().to(default_response))
    )
    .bind(format!("{}:{}", IP_ADDR, PORT))?
    .run()
    .await
}
