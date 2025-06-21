use std::process::Command;

fn main() {
    let status = Command::new("tailwindcss")
        .args([
            "-i", "static/css/tailwind.css",
            "-o", "static/gen/main.css",
        ])
        .status();

    match status {
        Ok(status) if status.success() => println!("TailwindCSS compiled successfully."),
        Ok(status) => eprintln!("TailwindCSS failed: {}", status),
        Err(err) => eprintln!("Failed to run tailwindcss: {}", err),
    }
}