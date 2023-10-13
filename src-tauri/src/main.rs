// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn echo(req: &str) -> String {
   let req = format!("Wow, Recieved!! {}!", req);
   println!("{}",&req);
   return req;
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![echo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

mod test {
    // type GenericError = Box<dyn std::error::Error + Send + Sync + 'static>;
    // type GenericResult<T> = Result<T,GenericError>;
    // use etcd_client::{Client, Error};
    // use futures::{executor, future}; 
    // #[test]
    // fn test_etcd(){
        
    // }
}