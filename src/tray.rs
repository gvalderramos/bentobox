use tray_item::{TrayItem, IconSource};
use std::sync::mpsc;

use crate::projects::{get_projects};

enum Message {
    Quit,
}

pub fn run_tray_app() {
    let img = image::load_from_memory(include_bytes!("../resource/bentobox.png")).unwrap().to_rgba8();
    let (width, height) = img.dimensions();
    let mut buf_icon = img.into_raw(); // Vec<u8> RGBA

    // swap R and B to get BGRA which tray_item expects
    for px in buf_icon.chunks_exact_mut(4) {
        px.swap(0, 1);
    }

    let icon = IconSource::Data {
        data: buf_icon,
        width: width as i32,
        height: height as i32,
    };

    let mut tray = TrayItem::new(
        "BentoBox",
        icon,
    )
    .unwrap();  

    let projects = get_projects();
    let ide_command = projects.user_config.bentobox.ide_command.clone();
    for project in projects.projects {
        // println!("Project: {} at {}", project.name, project.path);  
        let value = ide_command.clone(); 
        tray.add_menu_item(&project.name.to_string(), move || {
            project.open(value.as_str());
        }).unwrap();
    }

    tray.inner_mut().add_separator().unwrap();

    let (tx, rx) = mpsc::sync_channel(1);

    let quit_tx = tx.clone();
    tray.add_menu_item("Quit", move || {
        quit_tx.send(Message::Quit).unwrap();
    })
    .unwrap();

    loop {
        match rx.recv() {
            Ok(Message::Quit) => {
                break;
            }
            _ => {}
        }
    }
}