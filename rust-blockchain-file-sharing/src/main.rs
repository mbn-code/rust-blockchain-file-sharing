// main.rs

mod user_authentication;
mod file_sharing;
mod file;

fn main() {
    let username = "user1";
    let password = "password1";

    let file_to_share = file::File {
        name: "example.txt".to_string(),
        content: "This is the content of the file.".to_string(),
    };

    file_sharing::share_file(username, password, file_to_share);
}

