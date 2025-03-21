// make_index.rs

use std::fs::{self, File};
use std::io::{self, Write, Read};
use std::path::Path;

/// Generates an index.html file with the list of files and directories in the bucket folder
pub fn update_index_html(bucket_path: &str) -> io::Result<()> {
    let bucket = Path::new(bucket_path);
    if !bucket.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Bucket folder does not exist or is not a directory"));
    }

    // Recursively process files and directories
    fn process_dir(path: &Path, depth: usize, bucket: &Path) -> io::Result<String> {
        if depth > 3 { return Ok(String::new()); } 

        let entries = match fs::read_dir(path) {
            Ok(entries) => entries.filter_map(|entry| entry.ok()).collect::<Vec<_>>(),
            Err(_) => return Err(io::Error::new(io::ErrorKind::Other, "Unable to read directory")),
        };

		let mut entries = entries;
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        let mut html_content = String::new();
        let mut has_subdirectories = false;

        // Add files as links
        for entry in entries.iter().filter(|entry| entry.path().is_file()) {
            let file_name = entry.file_name().to_string_lossy().into_owned();
            match entry.path().strip_prefix(bucket) {
                Ok(relative_path) => {
                    html_content.push_str(&format!(
                        "<li><a href=\"/{0}\">{1}</a></li>",
                        relative_path.display(),
                        file_name
                    ));
                }
                Err(_) => {}
            }
        }

        // Add directories as headers
        for entry in entries.iter().filter(|entry| entry.path().is_dir()) {
            let dir_name = entry.file_name().to_string_lossy().into_owned();
            html_content.push_str(&format!("<li><h3>{0}/</h3><ul>", dir_name));
            let dir_path = entry.path();
            html_content.push_str(&process_dir(&dir_path, depth + 1, bucket)?);
            html_content.push_str("</ul></li>");
            has_subdirectories = true;
        }

        // If there are subdirectories, wrap the content in a <ul> for better structure
        if has_subdirectories {
            html_content = format!("<ul>{}</ul>", html_content);
        }

        Ok(html_content)
    }

    let bucket_html_content = process_dir(bucket, 0, bucket)?;

    let index_html_path = bucket.join("index.html");

	let mut existing_html = String::new();
   	if let Ok(mut file) = File::open(&index_html_path) {
  	    file.read_to_string(&mut existing_html)?;
 	}

	let start_tag = "<nav id=\"bucketList\">";
    let end_tag = "</nav>";
    let new_html = if let Some(start_idx) = existing_html.find(start_tag) {
        let end_idx = existing_html.find(end_tag).unwrap_or(start_idx);
        let updated_content = format!("{}{}{}", 
            &existing_html[..start_idx + start_tag.len()],
            bucket_html_content, 
            &existing_html[end_idx..]
        );
        updated_content
    } else {

	    let html_template = format!(
			r#"<!DOCTYPE html>
	<html lang="en">
	  <head>
		<meta charset="UTF-8">
		<meta name="viewport" content="width=device-width, initial-scale=1.0">
		<title>Rust Bucket</title>
	  </head>
	  <body>
		<h1>Bucket:</h1>
		<nav id="bucketList">
		    {}
		</nav>
		<footer>Rust Bucket File Server by Cursebreakers LLC, 2025</footer>
	  </body>
	</html>"#,
		   bucket_html_content
        );
        html_template
    };

    let mut file = File::create(index_html_path)?;
    file.write_all(new_html.as_bytes())?;

    Ok(())
}

