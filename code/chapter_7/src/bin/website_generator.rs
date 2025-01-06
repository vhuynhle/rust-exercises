use core::panic;
use std::{
    io::Write,
    path::{Path, PathBuf},
};

struct SiteSettings {
    name: String,
    author: String,
    create_js_folder: bool,
    create_css_folder: bool,
}

fn main() {
    let site_settings = read_site_settings();

    let cwd = std::env::current_dir().expect("Cannot detect current directory");

    let project_path = create_subdir(&cwd, &site_settings.name);

    let index_path = project_path.join("index.html");
    let mut index_file =
        std::fs::File::create_new(&index_path).expect("Failed to create index.html");
    let contents = get_index_page_contents(&site_settings.name, &site_settings.author);
    writeln!(index_file, "{}", contents).expect("Failed to write index.html");
    println!("Created {}", index_path.display());

    if site_settings.create_js_folder {
        create_subdir(&project_path, "js");
    }

    if site_settings.create_css_folder {
        create_subdir(&project_path, "css");
    }
}

fn create_subdir(path: &Path, subdir: &str) -> PathBuf {
    let subpath = path.join(subdir);
    std::fs::create_dir(&subpath).unwrap_or_else(|err| {
        panic!(
            "Failed to create {}: {}",
            subpath.display(),
            err.to_string()
        )
    });
    println!("Created {}", subpath.display());
    subpath
}

fn get_index_page_contents(site_name: &str, author: &str) -> String {
    format!(
        "
<html>
    <head>
        <title>{}</title>
        <meta name=\"author\" content=\"{}\">
    </head>
</html>
",
        site_name, author
    )
}

fn read_site_settings() -> SiteSettings {
    print!("Site name: ");
    let _ = std::io::stdout().flush();

    let mut site_name = String::new();
    std::io::stdin()
        .read_line(&mut site_name)
        .expect("Failed to read site name");
    site_name = site_name.trim().to_string();
    if site_name.is_empty() {
        eprintln!("Empty site name");
        std::process::exit(1);
    }

    print!("Author: ");
    let _ = std::io::stdout().flush();
    let mut author = String::new();
    std::io::stdin()
        .read_line(&mut author)
        .expect("Failed to read author name");
    author = author.trim().to_string();
    if author.is_empty() {
        eprintln!("Empty author name");
        std::process::exit(1);
    }

    let mut yes_no = String::new();
    print!("Create a folder for JavaScript (y/n)? ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut yes_no)
        .expect("Failed to read option");
    let create_js_folder = yes_no.trim().to_lowercase() == "y";

    yes_no.clear();
    print!("Create a folder for css (y/n)? ");
    let _ = std::io::stdout().flush();
    std::io::stdin()
        .read_line(&mut yes_no)
        .expect("Failed to read option");
    let create_css_folder = yes_no.trim().to_lowercase() == "y";

    SiteSettings {
        name: site_name,
        author,
        create_js_folder,
        create_css_folder,
    }
}
