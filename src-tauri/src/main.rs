mod crypto;
mod storage;
mod index;
mod models;

#[derive(Clone, serde::Serialize)]
pub struct FileEntry {
    pub name: String,
    pub size: u64,
    pub is_dir: bool,
}

#[tauri::command]
async fn get_files(path: String) -> Result<Vec<FileEntry>, String> {
    let _ = path;
    Ok(vec![
        FileEntry {
            name: "vault_demo.enc".to_string(),
            size: 2048,
            is_dir: false,
        },
        FileEntry {
            name: "Keys".to_string(),
            size: 0,
            is_dir: true,
        },
    ])
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![get_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
