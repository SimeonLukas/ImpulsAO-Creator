extern crate json_value_merge;
use std::io;
use std::path::Path;
use serde::{Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::fs;
use serde_json::Value;
use json_value_merge::Merge;
use image::io::Reader as ImageReader;
use image::imageops::*;
use std::path::PathBuf;
use std::sync::mpsc;
use image_compressor::FolderCompressor;
use image_compressor::Factor;


#[derive(Debug, Serialize)]
struct Impuls {
    date: String,
    r#type: String,
    title: String,
}

#[derive(Debug, Serialize)]
struct Description {
    date: String,
    r#type: String,
    text: String,
    team: String,
}


fn main() {
println!(r"
____ __  ___ ____   __  __ __    _____  ___    ____ 
/  _//  |/  // __ \ / / / // /   / ___/ /   |  / __ \
/ / / /|_/ // /_/ // / / // /    \__ \ / /| | / / / /
_/ / / /  / // ____// /_/ // /___ ___/ // ___ |/ /_/ / 
/___//_/_ /_//_/     \____//_____//____//_/  |_|\____/  
/ ____/_____ ___   ____ _ / /_ ____   _____           
/ /    / ___// _ \ / __ `// __// __ \ / ___/           
/ /___ / /   /  __// /_/ // /_ / /_/ // /               
\____//_/    \___/ \__,_/ \__/ \____//_/                
                                                    
");

    if Path::new("output").exists() != true {
        fs::create_dir("output");
    }
    if Path::new("output/img").exists() != true {
        fs::create_dir("output/img");
    }
    if Path::new("output/impuls").exists() != true {
        fs::create_dir("output/impuls");
    }
    if Path::new("output/data").exists() != true {
        fs::create_dir("output/data");
    }
    
    let mut dir = String::new();
    let mut vec = Vec::new();
    let mut vecextra = Vec::new();
    println!("In welchem Ordner liegen die Impulse?");
    io::stdin()
        .read_line(&mut dir)
        .expect("Diesen Ordner kenne ich nicht!");

    dir = dir.trim().to_string();

    
let path = Path::new(&dir);
let mut  number = 1;
for entry in path.read_dir().expect("read_dir Fehler") {
    if let Ok(entry) = entry {
        let filepath = entry.path();
        let basename = filepath.file_stem().unwrap();
        let basename = basename.to_str().expect("REASON").to_string();
        println!("Bild {} wird verarbeitet!", basename);
       
        if filepath.to_str().expect("REASON").to_string().contains("team"){
            let description = Description{
                date: String::from("00000000"),
                r#type: String::from("description"),
                text: String::from("<h1>Auf dem Weg nach Bethlehem</h1> <p lang=\"de\">Wenn wir morgens aufwachen, dann sind wir schon mitten drin in unserem Alltag. Gewohntes, Stressiges, Überraschendes, Angenehmes und Vieles mehr warten auf uns. Doch oft bleibt nicht immer die Zeit, darin das Schöne und Herausfordernde wahrzunehmen. Aus diesem Grund wollen wir mit unserem Impulskalender unserem Alltag ein bischen mehr Zeit geben. Zweimal im Jahr gestaltet das Impulskalenderteam kurze Texte, die den Alltag unter einem neuen Blick zeigen: Einmal vom 1. Advent bis zum Ende der Weihnachtszeit, und einmal durch die Fastenzeit bis Ostern. Daher kommt auch das „AO“ im Namen der Homepage: Advent und Ostern, von Anfang bis Ende. Die Impulse verschicken wir per Email oder via Smartphone App. Wir freuen uns auf Sie. Ihr Impulskalenderteam</p><p> Falls Ihnen die App gefällt, dann können Sie sie gerne <a href='#' onclick='window.plugins.socialsharing.share(`Diese App möchte ich gerne mit Dir teilen: ImpulsAO für IOS & Android https://onelink.to/4y2ff5`, null, null, null);' >teilen.</a></p>"),
                team: basename,
            };
            vecextra.push(description);
            println!("Beschreibung wurde geschrieben!");
        }
        else{
        let img = image::open(filepath).unwrap();
        let img = resize(&img, 713, 1000, Nearest);
        let img = blur(&img, 10.0);
        let img_file = format!("output/img/{basename}.jpg");
        img.save(img_file);
        let name = format!("{number}. Impuls");
        let impuls = Impuls{
            date: basename,
            r#type: String::from("text"),
            title: name,
        };
        vec.push(impuls);
        number += 1;
    }
        

    }
}


let origin = PathBuf::from(&dir);   // original directory path
let dest = PathBuf::from("output/impuls");       // destination directory path
let thread_count = 4;                       // number of threads
let (tx, tr) = mpsc::channel();             // Sender and Receiver. for more info, check mpsc and message passing. 

let mut comp = FolderCompressor::new(origin, dest);
comp.set_cal_func(|width, height, file_size| {return Factor::new(75., 0.7)});
comp.set_thread_count(4);
comp.set_sender(tx);

match comp.compress(){
    Ok(_) => {},
    Err(e) => println!("Cannot compress the folder!: {}", e),
}




let json = serde_json::to_string(&vec).unwrap();
let jsonextra = serde_json::to_string(&vecextra).unwrap();
let mut json1: Value= serde_json::from_str(&json).unwrap();
let json2: Value= serde_json::from_str(&jsonextra).unwrap();
json1.merge(json2);
let jsonfinal = serde_json::to_string_pretty(&json1).unwrap();
println!{"{:?}", jsonfinal};
let dir = format!("output/data/{dir}.json");
std::fs::write(
    dir,
    jsonfinal
).unwrap();
println!("Daten für den Server wurden erfolgreich erstellt!");
}
