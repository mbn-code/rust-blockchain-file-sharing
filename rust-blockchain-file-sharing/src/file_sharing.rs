// file_sharing.rs

use crate::user_authentication::authenticate_user;
use crate::file::File;

pub fn share_file(username: &str, password: &str, file: File) {
    if authenticate_user(username, password) {
        // Simulate sending the file to another system
        send_file_to_another_system(file);
        println!("File shared successfully!");
    } else {
        println!("Authentication failed. Unable to share the file.");
    }
}

fn send_file_to_another_system(file: File) {
    // Simulate sending the file to another system (print for demonstration purposes)
    println!("File sent to another system: {:?}", file);
}
