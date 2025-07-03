use notify::{RecommendedWatcher, RecursiveMode, Result as NotifyResult, Watcher, EventKind};
use std::sync::mpsc::channel;
use std::path::Path;
use std::io::{self, Write};
use std::fs;
use fs_extra::file::{copy, CopyOptions};
use log::{info, error};
use notify_rust::Notification;

fn main() -> NotifyResult<()> {
    env_logger::init();

    let mut watch_path = String::from("./gta5_folder");
    let mut source_file = String::from("./redux_mod/file_to_copy.txt");
    let mut dest_file = String::from("./gta5_folder/file_to_copy.txt");

    loop {
        println!("\nТекущее расположение:");
        println!("1. Папка GTA5: {}", watch_path);
        println!("2. Файл редукса: {}", source_file);
        println!("3. Куда копировать: {}", dest_file);
        println!("4. Запустить автозагрузку");
        println!("5. Запустить слежение и копирование");
        println!("6. Выйти");
        print!("Выберите действие: ");
        io::stdout().flush().unwrap();
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        match choice.trim() {
            "1" => {
                print!("Введите путь к папке GTA5: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                watch_path = input.trim().to_string();
            },
            "2" => {
                print!("Введите путь к файлу редукса: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                source_file = input.trim().to_string();
            },
            "3" => {
                print!("Введите путь, куда копировать: ");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                dest_file = input.trim().to_string();
            },
            "4" => {
                if let Err(e) = setup_autostart() {
                    println!("Ошибка автозагрузки: {}", e);
                } else {
                    println!("Добавлено в автозагрузку!");
                }
            },
            "5" => {
                run_watcher(&watch_path, &source_file, &dest_file)?;
            },
            "6" => break,
            _ => println!("Неизвестная команда"),
        }
    }
    Ok(())
}

fn run_watcher(watch_path: &str, source_file: &str, dest_file: &str) -> NotifyResult<()> {
    let (tx, rx) = channel();
    let mut watcher: RecommendedWatcher = RecommendedWatcher::new(tx, notify::Config::default())?;
    watcher.watch(Path::new(watch_path), RecursiveMode::NonRecursive)?;

    info!("Watching for changes in {}", watch_path);
    println!("Watching for changes in {}", watch_path);

    loop {
        match rx.recv() {
            Ok(event) => {
                if let Ok(event) = event {
                    if matches!(event.kind, EventKind::Modify(_)) {
                        info!("Change detected! Copying file...");
                        println!("Change detected! Copying file...");
                        let options = CopyOptions::new();
                        match copy(source_file, dest_file, &options) {
                            Ok(_) => {
                                println!("Файл успешно заменён!");
                                let _ = Notification::new()
                                    .summary("Redux заменён!")
                                    .body("Файл успешно скопирован.")
                                    .show();
                            },
                            Err(e) => {
                                error!("Failed to copy file: {}", e);
                                let _ = Notification::new()
                                    .summary("Ошибка копирования Redux")
                                    .body(&format!("Ошибка: {}", e))
                                    .show();
                            },
                        }
                    }
                }
            }
            Err(e) => error!("Watch error: {:?}", e),
        }
    }
}

fn setup_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let home = std::env::var("HOME")?;
    let autostart_dir = format!("{}/.config/autostart", home);
    fs::create_dir_all(&autostart_dir)?;
    let desktop_file = format!("{}/qwertredux.desktop", autostart_dir);
    let exec_path = std::env::current_exe()?;
    let content = format!("[Desktop Entry]\nType=Application\nName=QwertRedux\nExec={}\nX-GNOME-Autostart-enabled=true\n", exec_path.display());
    fs::write(desktop_file, content)?;
    Ok(())
}
