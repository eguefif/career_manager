use std::process::Command;
fn main() {
    match Command::new("/bin/sh")
        .arg("./html/website/dist/run_docker.sh")
        .status()
    {
        Ok(status) => {
            if status.success() {
                println!("It worked!");
            } else {
                println!("Failed");
            }
        }
        Err(e) => println!("Error: {:?}", e),
    }
}
