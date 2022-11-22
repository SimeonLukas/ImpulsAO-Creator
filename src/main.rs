extern crate json_value_merge;
use image::imageops::*;
use image::GenericImageView;
use image_compressor::Factor;
use image_compressor::FolderCompressor;
use json_value_merge::Merge;
use serde::Serialize;
use serde_json::Value;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use std::sync::mpsc;

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
    println!(
        r"
    ____ __  ___ ____   __  __ __    _____  ___    ____     Y^^~~~^.    ~^^:                :^~~~^^Y
   /  _//  |/  // __ \ / / / // /   / ___/ /   |  / __ \    ~^~~~:     ~!~~~:                 :~~~^~
   / / / /|_/ // /_/ // / / // /    \__ \ / /| | / / / /    ^~~~.     ~7~^~~~.       .....     .~~^!
 _/ / / /  / // ____// /_/ // /___ ___/ // ___ |/ /_/ /     ~~~.     ^?.~!^~~~.   ^!!.  .!!:    :~^!
/___//_/_ /_//_/     \____//_____//____//_/  |_|\____/      ~~:     :?:  7~^^~~  ^7~.    .7!^    ^^!
  / ____/_____ ___   ____ _ / /_ ____   _____               ~~.    .?^   .7~^^~~ .7!.    .?~.    :~!
 / /    / ___// _ \ / __ `// __// __ \ / ___/               ~~.   7!^~~~~~~~~~~^~~^^!~  !!^~!.   :~!   
/ /___ / /   /  __// /_/ // /_ / /_/ // /                   ~~^  !!         ^!~~^~:              ^^!
\____//_/    \___/ \__,_/ \__/ \____//_/                    ~~~.^7.          ~!~~~~.            :~^!
                                                            ^~~~!.           .7~~~~~:          :~~^!
                                                            ~^~~~:         ^^^^^::::^^^.      ^~~~^~
Aufbereitungstool für die ImpulsAO-APP und deren Server.
Zur Aufbereitung benötigst du alle Impulse in einem Ordner.

Bitte benenne den Ordner je nach Jahr (Beispiel: 2022) und der Kalendernummer des Jahres (01 Fastenzeit bzw. 02 Advent).
Der Ordner für die Impulse der Fastenzeit 2025 würde folgenden Namen tragen: 202501
Der Ordner für die Impulse der Adventszeit 2024 würde folgenden Namen tragen: 202402

In dem Ordner sollen nur die Bilder der Impulse liegen. Diese Bilder sind je nach Datum benannt.
Hier ein Beispiel:
Der Impuls für den 1. Dezember 2024 trägt den Namen 20241201.jpg
Der Impuls für den 2. Januar 2025 trägt den Namen 20250102.jpg

Das Foto der Autoren sollte dann folgenden Titel tragen: 'Ordnername'-team.jpg
Das Foto der Autoren vom Adventskalender 2024 würde folgenden Namen tragen: 202402-team.jpg

Die Impuls_ao_Creator.exe muss im gleichen Ordner liegen wie der Ordner mit den Impulsen.

Alles ❤ ,
Simeon
"
    );
    converter();
}

fn converter() {
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
    let mut theme = String::new();
    let mut answer = String::new();
    let mut vec = Vec::new();
    let mut vecextra = Vec::new();
    println!(
        "In welchem Ordner liegen die Impulse?"
    );
    io::stdin()
        .read_line(&mut dir)
        .expect("Diesen Ordner kenne ich nicht!");
    dir = dir.trim().to_string();

    if Path::new(&dir).exists() != true {
        println!("Ordner leider nicht vorhanden!");
        converter();
    }

    println!("Wie lautet das Thema des Kalenders?");
    io::stdin()
        .read_line(&mut theme)
        .expect("Thema nicht deutlich!");

    theme = theme.trim().to_string();

    let path = Path::new(&dir);
    let mut number = 1;
    for entry in path.read_dir().expect("read_dir Fehler") {
        if let Ok(entry) = entry {
            let filepath = entry.path();
            let basename = filepath.file_stem().unwrap();
            let basename = basename.to_str().expect("REASON").to_string();
            println!("Bild {} wird verarbeitet!", basename);

            if filepath
                .to_str()
                .expect("REASON")
                .to_string()
                .contains("team")
            {
                let description_text = format!("<h1>{theme}</h1> <p lang=\"de\">Wenn wir morgens aufwachen, dann sind wir schon mitten drin in unserem Alltag. Gewohntes, Stressiges, Überraschendes, Angenehmes und Vieles mehr warten auf uns. Doch oft bleibt nicht immer die Zeit, darin das Schöne und Herausfordernde wahrzunehmen. Aus diesem Grund wollen wir mit unserem Impulskalender unserem Alltag ein bischen mehr Zeit geben. Zweimal im Jahr gestaltet das Impulskalenderteam kurze Texte, die den Alltag unter einem neuen Blick zeigen: Einmal vom 1. Advent bis zum Ende der Weihnachtszeit, und einmal durch die Fastenzeit bis Ostern. Daher kommt auch das „AO“ im Namen der Homepage: Advent und Ostern, von Anfang bis Ende. Die Impulse verschicken wir per Email oder via Smartphone App. Wir freuen uns auf Sie. Ihr Impulskalenderteam</p><p> Falls Ihnen die App gefällt, dann können Sie sie gerne <a href='#' onclick='window.plugins.socialsharing.share(`Diese App möchte ich gerne mit Dir teilen: ImpulsAO für IOS & Android https://onelink.to/4y2ff5`, null, null, null);' >teilen.</a></p>");
                let description = Description {
                    date: String::from("00000000"),
                    r#type: String::from("description"),
                    text: description_text,
                    team: basename,
                };
                vecextra.push(description);
                println!("Beschreibung wurde geschrieben!");
            } else {
                let img = image::open(filepath).unwrap();
                let (width, height) = img.dimensions();
                let img = resize(&img, width * 2000 / height, 2000, Nearest);
                let impuls_file = format!("output/impuls/{basename}.jpg");
                img.save(impuls_file);

                let img = resize(&img, width * 750 / height, 750, Nearest);
                let img_file = format!("output/img/{basename}.jpg");
                img.save(img_file);
                let name = format!("{number}. Impuls");
                let impuls = Impuls {
                    date: basename,
                    r#type: String::from("text"),
                    title: name,
                };
                vec.push(impuls);
                number += 1;
            }
        }
    }

    let origin = PathBuf::from("output/impuls");
    let dest = PathBuf::from("output/impuls");
    let thread_count = 4;
    let (tx, tr) = mpsc::channel();

    let mut comp = FolderCompressor::new(origin, dest);
    comp.set_cal_func(|width, height, file_size| return Factor::new(75., 0.7));
    comp.set_thread_count(4);
    comp.set_sender(tx);

    match comp.compress() {
        Ok(_) => {}
        Err(e) => println!("Cannot compress the folder!: {}", e),
    }

    let origin = PathBuf::from("output/img");
    let dest = PathBuf::from("output/img");
    let thread_count = 4;
    let (tx, tr) = mpsc::channel();

    let mut comp = FolderCompressor::new(origin, dest);
    comp.set_cal_func(|width, height, file_size| return Factor::new(75., 0.7));
    comp.set_thread_count(4);
    comp.set_sender(tx);

    match comp.compress() {
        Ok(_) => {}
        Err(e) => println!("Cannot compress the folder!: {}", e),
    }

    let json = serde_json::to_string(&vec).unwrap();
    let jsonextra = serde_json::to_string(&vecextra).unwrap();
    let mut json1: Value = serde_json::from_str(&json).unwrap();
    let json2: Value = serde_json::from_str(&jsonextra).unwrap();
    json1.merge(json2);
    let jsonfinal = serde_json::to_string_pretty(&json1).unwrap();
    let dir = format!("output/data/{dir}.json");
    std::fs::write(dir, jsonfinal).unwrap();
    println!("Daten für den Server wurden erfolgreich erstellt!");
    println!("Möchtest du weitermachen?");
    io::stdin()
        .read_line(&mut answer)
        .expect("Diesen Ordner kenne ich nicht!");
    answer = answer.trim().to_string();

    if answer != "nein" {
        converter();
    }
}
