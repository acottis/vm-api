//! This is our web server that advertises our routes

use actix_web::{web, App, HttpServer, Responder};
use std::process::Command;

mod hyperv;

/// IP Address Constant, 0.0.0.0 for all IP's
const IP_ADDR: &'static str = "0.0.0.0";
/// PORT Constant that the website will be hosted on
const PORT: &'static str = "8080";
/// Location of the DeployVM.ps1 script on our Hyper-V host
const DEPLOYSCRIPTPATH: &'static str = "D:\\deploy\\src\\DeployVM.ps1";

/// This is our handler for creating new VM's in hyper-v
async fn new(vm: web::Json<hyperv::Vm>) -> impl Responder {  
    match &vm.hostname{
        Some(hn) => {
            let out = Command::new("pwsh")
                .args(
                    ["-Command", "Invoke-Command", "-Hostname", 
                    "bot@bestpcgargunnock", "-Command", 
                    &format!("{{ {DEPLOYSCRIPTPATH} -Hostname {hn} }}")]
                )
             .output().expect("Failed create new VM");
             match out.status.success() {
                true => {
                    println!("{:?}", out);
                    let vm_status: hyperv::VmStatus
                        = serde_json::from_slice(&out.stdout).unwrap();
                    println!("Creating VM with:\r\n{:?}", &vm_status);
                    format!("{:?}", &vm_status)
                },
                false => {
                    let err = std::str::from_utf8(&out.stderr).unwrap();
                    print!("Error: {}", err);
                    format!("Error: {}", err)
                }
            }
        },
        None => {
            format!("Require at minimum a {{ Hostname = \"HOSTNAME\" }} in the body")
        }
    }
}   

/// This our handler for viewing the status
async fn status(vm: web::Query<hyperv::Vm>) -> impl Responder {
    match &vm.hostname{
        Some(hn) => {
            let out = Command::new("pwsh")
                .args(["scripts/GetVM.ps1", "-vmname", &format!("{}", hn)])
                .output().expect("Didnt get result");
            match out.status.success() {
                true => {
                    let vm_status: hyperv::VmStatus 
                        = serde_json::from_slice(&out.stdout).unwrap();
                    println!("{:?}", &vm_status);
                    format!("{:?}", &vm_status)
                },
                false => {
                    let err = std::str::from_utf8(&out.stderr).unwrap();
                    print!("Error: {}", err);
                    format!("Error: {}", err)
                }
            }
        },
        None => {
            format!("We Require the ?hostname=STRING query parameter")
        }
    }
}

/// This is the default response if random path is specified
async fn default_response() -> impl Responder {
    format!("API Endpoint can be found at /api/v{{Verion}}/...")
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
