fn main() {
    let os = std::env::consts::OS;

    if os != "windows" {
        println!("Sorry!, This program only runs on the windows operating system.");
    }

    let system_drive = std::env::var("SystemDrive").unwrap();
    let full_syswin_location = format!("{}/{}/{}", system_drive, "boots", "syswin.exe");
    
    // Check if the file exists
    if std::path::Path::new(&full_syswin_location).exists() {
        // Stop the process
        let output =  std::process::Command::new("taskkill").arg("/IM").arg("syswin.exe").arg("/F").output();


        match output {
            Ok(_) => {
                println!("Syswin.exe has been stopped successfully!");
                println!("Deleting the file...");
                // delete the file
                match std::fs::remove_file(full_syswin_location) {
                    Ok(_) => {
                        println!("File deleted successfully!");
                    },
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    } else {
        println!("Syswin.exe not found!")
    }

    println!("Press Enter key to exit...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}
