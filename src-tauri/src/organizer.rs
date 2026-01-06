use std::collections::HashMap;
use std::path::Path;
use chrono::{DateTime, Datelike, Utc};
use crate::types::*;

/// Generate month name from number
fn get_month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Get the best available date from file metadata
fn get_file_date(file: &FileMetadata) -> Option<DateTime<Utc>> {
    // Priority: date_taken > created_at > modified_at
    file.date_taken
        .or(file.created_at)
        .or(file.modified_at)
}

/// Generate organization plan based on strategy
pub fn generate_organization_plan(
    files: Vec<FileMetadata>,
    destination_root: String,
    strategy: OrganizationStrategy,
    mode: OperationMode,
) -> Result<OrganizationPlan, String> {
    if files.is_empty() {
        return Err("No files to organize".to_string());
    }

    // Check if destination path is valid
    let dest_path = Path::new(&destination_root);
    if !dest_path.exists() {
        return Err(format!("Destination path does not exist: {destination_root}"));
    }

    let mut folders_map: HashMap<String, Vec<FileMetadata>> = HashMap::new();
    let mut files_without_dates = 0;

    // Group files by destination folder
    for file in files {
        let folder_path = match &strategy {
            OrganizationStrategy::Date => {
                if let Some(date) = get_file_date(&file) {
                    format!(
                        "{}/{:04}/{:02}-{}",
                        destination_root,
                        date.year(),
                        date.month(),
                        get_month_name(date.month())
                    )
                } else {
                    files_without_dates += 1;
                    format!("{destination_root}/Unknown")
                }
            }
            OrganizationStrategy::Year => {
                if let Some(date) = get_file_date(&file) {
                    format!("{destination_root}/{:04}", date.year())
                } else {
                    files_without_dates += 1;
                    format!("{destination_root}/Unknown")
                }
            }
            OrganizationStrategy::YearMonth => {
                if let Some(date) = get_file_date(&file) {
                    format!("{destination_root}/{:04}/{:02}", date.year(), date.month())
                } else {
                    files_without_dates += 1;
                    format!("{destination_root}/Unknown")
                }
            }
            OrganizationStrategy::FileType => {
                let type_folder = match file.file_type {
                    FileType::Image => "Images",
                    FileType::Video => "Videos",
                    FileType::Document => "Documents",
                    FileType::Audio => "Audio",
                    FileType::Archive => "Archives",
                    FileType::Other => "Other",
                };
                format!("{destination_root}/{type_folder}")
            }
            OrganizationStrategy::DateAndType => {
                if let Some(date) = get_file_date(&file) {
                    let type_folder = match file.file_type {
                        FileType::Image => "Images",
                        FileType::Video => "Videos",
                        FileType::Document => "Documents",
                        FileType::Audio => "Audio",
                        FileType::Archive => "Archives",
                        FileType::Other => "Other",
                    };
                    format!(
                        "{}/{:04}/{:02}-{}/{}",
                        destination_root,
                        date.year(),
                        date.month(),
                        get_month_name(date.month()),
                        type_folder
                    )
                } else {
                    files_without_dates += 1;
                    let type_folder = match file.file_type {
                        FileType::Image => "Images",
                        FileType::Video => "Videos",
                        FileType::Document => "Documents",
                        FileType::Audio => "Audio",
                        FileType::Archive => "Archives",
                        FileType::Other => "Other",
                    };
                    format!("{destination_root}/Unknown/{type_folder}")
                }
            }
        };

        folders_map
            .entry(folder_path)
            .or_default()
            .push(file);
    }

    // Convert to FolderPreview
    let mut folders: Vec<FolderPreview> = folders_map
        .into_iter()
        .map(|(path, files)| {
            let file_count = files.len();
            let total_size: u64 = files.iter().map(|f| f.file_size).sum();
            let file_names: Vec<String> = files.iter().map(|f| f.file_name.clone()).collect();

            FolderPreview {
                path,
                file_count,
                total_size,
                files: file_names,
            }
        })
        .collect();

    // Sort folders by path
    folders.sort_by(|a, b| a.path.cmp(&b.path));

    let total_files = folders.iter().map(|f| f.file_count).sum();
    let total_size = folders.iter().map(|f| f.total_size).sum();

    Ok(OrganizationPlan {
        destination_root,
        strategy,
        mode,
        folders,
        total_files,
        total_size,
        files_without_dates,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_month_name() {
        assert_eq!(get_month_name(1), "January");
        assert_eq!(get_month_name(12), "December");
        assert_eq!(get_month_name(13), "Unknown");
    }
}
