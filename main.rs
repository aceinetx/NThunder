use pancurses;
use pancurses::*;
use std::fs;
use std::fs::{metadata};
use serde_json::Value;
use std::env;

static TITLE_SIZE: i32 = 3;

// #[derive(Clone)] to implement clone trait
#[derive(Clone)]
struct FsEntry {
    name: String,
    absolute_path: String,
    is_folder: bool,
    reversed: bool,
    hide: bool,
}

fn get_slash() -> String {
    let os_type = env::consts::OS;
    if os_type == "windows" {
        return String::from("\\");
    }
    return String::from("/");
}

fn get_slash_char() -> char {
    get_slash().chars().collect::<Vec<_>>()[0]
}

// File types definitions in json bcuz I was lazy
static FILE_TYPES: &str = r#"
{
    "txt": "Text file",
    "py": "Python script",
    "c": "C Source file",
    "cpp": "C++ Source file",
    "h": "Cxx header source file",
    "exe": "Windows Executable",
    "dll": "Windows Executable Extension",
    "sys": "Windows System Extension",
    "so": "Linux Executable Extension",
    "rs": "Rust Source file",
    "json": "JSON File",
    "js": "Javascript Source file",
    "ts": "Typescript Source file",
    "java": "Java Source file",
    "kt": "Kotlin Source file",
    "bat": "Windows Batch file",
    "cmd": "Windows Batch file",
    "sh": "Linux Bash Script file",
    "lua": "Lua Source file",
    "zip": "Zip Archive",
    "gz": "Gzip Archive",
    "": "Dotfile configuration",
    "wpp": "Watermelon++ Source file",
    "pyc": "Python Compiled file",
    "ct": "Cheat engine cheat sheet",
    "xlsx": "Microsoft Excel Table",
    "pptx": "Microsoft Powerpoint Slideshow",
    "pdf": "Microsoft Powerpoint Slideshow",
    "toml": "TOML Configuration",
    "lua": "Lua Source file",
    "mk": "Makefile build",
    "png": "PNG Picture",
    "jpeg": "JPEG Picture",
    "log": "Logging file",
    "ini": "INI Configuration",
    "plr": "Terraria player file",
    "wld": "Terraria world file",
    "dat": "Bytecode data file",
    "html": "HTML Website page",
    "css": "Cascaded style sheet file",
    "pdb": "Program debug database",
    "xml": "Xml Source file",
    "7z": "7-Zip Archive",
    "lnk": "Windows Shortcut",
    "url": "Windows Website Shortcut",
    "bak": "Backup file",
    "tmp": "Temp file",
    "doc": "Microsoft Word file",
    "rtf": "Microsoft Word file",
    "vbs": "Windows Based Script Host file",
    "reg": "Windows Registry file"
}"#;

fn get_file_extension(filename: String) -> String{

    let mut extension = String::new();

    // Iterate through reversed string
    for chr in filename.chars().rev().collect::<String>().chars().collect::<Vec<_>>() {
        if chr == '.'{
            break
        }
        extension.push(chr);
    }

    // Return reversed string
    extension.chars().rev().collect::<String>().to_lowercase()
}

fn get_file_extension_definition(extension: String) -> String{
    //let last_version: Value = serde_json::from_str(x.as_str())?;
    let mut definition = String::new();

    let definitions: Value = serde_json::from_str(FILE_TYPES).unwrap();

    let result = definitions[extension].as_str();
    if result != None {
        definition = result.unwrap().to_string();
    }

    definition
}

fn fs_get_path_from_vector(path: Vec<String>) -> String{
    let mut path_str = String::from(get_slash().as_str());

    for path_entry in path {
        path_str.push_str(path_entry.as_str());
        path_str.push_str(get_slash().as_str());
    }

    path_str.push_str(get_slash().as_str());

    // Check if path_str is only "\", if it is make it empty
    if path_str.len() == 2 {
        path_str.pop();
    }

    path_str.replace(&(get_slash()+get_slash().as_str()), get_slash().as_str())
}

fn fs_find_file(entries: Vec<FsEntry>) -> usize {

    let mut idx = 0;
    for element in entries {
        if element.is_folder == true {
            return idx;
        }
        idx += 1
    }

    0
}

fn fs_sort_entries(entries: Vec<FsEntry>) -> Vec<FsEntry>{
    let mut buf = entries.clone();
    let mut res: Vec<FsEntry> = vec![];

    for _i in 0..entries.len(){
        let index = fs_find_file(buf.clone());
        res.push(buf[index].clone());
        buf.remove(index);
    }

    res
}

fn fs_get_files(path: Vec<String>) -> Vec<FsEntry> {

    let path_str = fs_get_path_from_vector(path);
    let mut entries: Vec<FsEntry> = vec![];

    let paths_result = fs::read_dir(path_str);
    if paths_result.is_err(){ // If error occured accessing the folder
        let mut entries_b: Vec<FsEntry> = vec![];
        entries_b.push(FsEntry{ hide: false, name: String::from(".."), absolute_path: String::from(".."), is_folder: true, reversed: true });
        entries_b.push(FsEntry{ hide: false, name: String::from("(No access to this folder)"), absolute_path: String::from("(No access to this folder)"), is_folder: true, reversed: true });
        return entries_b;
    }
    let paths = paths_result.unwrap();



    entries.push(FsEntry{ hide: false, name: String::from(".."), absolute_path: String::from(".."), is_folder: true, reversed: true });
    for path in paths {
        let raw_path = path.unwrap().path();
        let entry_path = raw_path.to_str().unwrap().to_string().replace(&(get_slash()+get_slash().as_str()), get_slash().as_str());
        let is_file_res = metadata(entry_path.clone());
        let mut is_file = true;
        let mut hide_file = false;
        if is_file_res.is_ok(){
            is_file = is_file_res.unwrap().is_file();
        } else {
            hide_file = true;
        }
        entries.push(FsEntry{ hide: hide_file, name: get_last_from_absolute_path(entry_path.clone()), absolute_path: entry_path.clone(), is_folder: !is_file, reversed: false });
    }

    entries = fs_sort_entries(entries);

    entries
}

fn get_last_from_absolute_path(path: String) -> String {
    let mut last = String::new();
    // Iterate through reversed string
    for chr in path.chars().rev().collect::<String>().chars().collect::<Vec<_>>(){
        if chr == get_slash_char() { break };
        last.push(chr);
    }

    // Return reversed string
    last.chars().rev().collect::<String>()
}

fn main() {

    let mut no_mouse_mode = false;
    let mut fake_mouse_x = 0;
    let mut fake_mouse_y = 0;

    let mut path: Vec<String> = vec![];
    let mut offset = 0;

    println!("NThunder v1.0 by aceinetx (2022-present, app: 2024-present)");

    let w = initscr();
    start_color();        // Make colors viewable
    noecho();             // No input feedback
    curs_set(0);

    // Initalize colors
    init_pair(1, 0, 15); // Hold color
    init_pair(2, 2, 0);  // File color
    init_pair(3, 0, 2);  // File hold color

    w.nodelay(true); // Make it no delay for constant updates


    w.keypad(true); // Make arrow keys working
    let mut temp: mmask_t = 0;
    mousemask(ALL_MOUSE_EVENTS, Some(&mut temp)); // Initalizing temp variable because
                                                  // ```mousemask(ALL_MOUSE_EVENTS, std::ptr::null_mut());```
                                                  // does not work due to fcking types

    loop {
        w.clear();

        let mouse_result = getmouse();
        let mut mouse = MEVENT{ x: 0, y: 0, z: 0, bstate: 0, id: -1 }; // Make empty mouse if not supported
        if mouse_result.is_ok() {
            mouse = mouse_result.unwrap();
        }

        if no_mouse_mode == true {
            mouse.x = fake_mouse_x;
            mouse.y = fake_mouse_y;
        }

        let mut entries_index = TITLE_SIZE;
        let mut files = fs_get_files(path.clone());
        if offset > files.len() as i32 - (w.get_max_y()-TITLE_SIZE) {
            offset = files.len() as i32 - (w.get_max_y()-TITLE_SIZE);
        }

        // Aimed entry - Entry that mouse pointer is pointing to
        let mut aimed_entry = FsEntry { hide: false, name: String::new(), is_folder: false, absolute_path: String::new(), reversed: true };
        let mut no_item_aimed = true;

        if offset < 0 {
            offset = 0;
        }

        w.mvprintw(w.get_max_y()-2, w.get_max_x()-1, "^");
        w.mvprintw(w.get_max_y()-1, w.get_max_x()-1, "v");

        w.attron(ColorPair(1));
        if mouse.x == w.get_max_x()-1 && (mouse.y == w.get_max_y()-1){
            w.mvprintw(w.get_max_y()-1, w.get_max_x()-1, "v");
        }
        if mouse.x == w.get_max_x()-1 && (mouse.y == w.get_max_y()-2){
            w.mvprintw(w.get_max_y()-2, w.get_max_x()-1, "v");
        }
        w.attroff(ColorPair(1));

        for entry in &mut files[offset as usize..]{
            if entry.hide == true {
                continue;
            }
            if entry.is_folder {
                w.attron(ColorPair(2));
            }
            w.mvprintw(entries_index, 0, &entry.name);
            if entry.is_folder {
                w.attroff(ColorPair(2));
            }
            if mouse.y == entries_index && (mouse.x <= entry.name.len() as i32+1){
                if !entry.is_folder {
                    w.attron(ColorPair(1));
                } else {
                    w.attron(ColorPair(3));
                }
                w.mvprintw(entries_index, 0, format!("{} <", &entry.name));
                if !entry.is_folder {
                    w.attroff(ColorPair(1));
                } else {
                    w.attroff(ColorPair(3));
                }
                aimed_entry.name = entry.name.clone();
                aimed_entry.absolute_path = entry.absolute_path.clone();
                aimed_entry.is_folder = entry.is_folder;
                no_item_aimed = false;
            }
            entries_index += 1;
        }

        w.mvprintw(0, 0, format!("{}", fs_get_path_from_vector( path.clone() ) ) );
        if no_item_aimed == false {
            w.mvprintw(2, 0, format!("Selected item: Folder: {}, Name: {}", aimed_entry.is_folder, aimed_entry.name));
            if !aimed_entry.is_folder{
                w.mvprintw(2, 0, format!("Selected item: Folder: {}, Name: {} ({})", aimed_entry.is_folder, aimed_entry.name, get_file_extension_definition(get_file_extension(aimed_entry.name.clone()))));
            }
        } else {
            w.mvprintw(2, 0, "(No item selected)");
        }
        w.mvprintw(1, 0, "----------------------------------------- NThunder");

        if no_mouse_mode == true {
            w.mvprintw(fake_mouse_y, fake_mouse_x, "@");
        }

        match w.getch() {

            Some(Input::KeyDown) => {
                offset += 1;
            }

            Some(Input::KeyUp) => {
                if offset >= 1 {
                    offset -= 1;
                }
            }

            Some(Input::KeyMouse) => {
                if no_item_aimed == false {
                    if aimed_entry.is_folder == false { // If file we can just skip everything
                        continue;
                    }
                    offset = 0;
                    if aimed_entry.name != ".." && (aimed_entry.name != "(No access to this folder)") {
                        path.push(aimed_entry.name.clone());
                    } else {
                        if aimed_entry.name == ".." {
                            if path.len() > 0 {
                                path.pop();
                            }
                        }
                    }
                } else {
                    // Offset buttons
                    if mouse.x == w.get_max_x()-1 && (mouse.y == w.get_max_y()-1){
                        offset += 1;
                    }
                    if mouse.x == w.get_max_x()-1 && (mouse.y == w.get_max_y()-2){
                        if offset >= 1 {
                            offset -= 1;
                        }
                    }
                }
            }

            Some(Input::Character('x')) => {
                if no_item_aimed == false {
                    if aimed_entry.is_folder == false { // If file we can just skip everything
                        continue;
                    }
                    offset = 0;
                    if aimed_entry.name != ".." && (aimed_entry.name != "(No access to this folder)") {
                        path.push(aimed_entry.name.clone());
                    } else {
                        if aimed_entry.name == ".." {
                            if path.len() > 0 {
                                path.pop();
                            }
                        }
                    }
                } else {
                    // Offset buttons
                    if mouse.x == w.get_max_x()-1 && (mouse.y == w.get_max_y()-1){
                        offset += 1;
                    }
                    if mouse.x == w.get_max_x()-1 && (mouse.y == w.get_max_y()-2){
                        if offset >= 1 {
                            offset -= 1;
                        }
                    }
                }
            }

            Some(Input::Character(c)) => {
                if c == 'q'{
                    break;
                } else if c == 'w'{
                    if offset >= 1 {
                        offset -= 1;
                    }
                } else if c == 's'{
                    offset += 1;
                } else if c == 'k'{
                    fake_mouse_y -= 1;
                    if fake_mouse_y < 0{
                        fake_mouse_y = 0;
                    }
                } else if c == 'j'{
                    fake_mouse_y += 1;
                } else if c == 'h'{
                    fake_mouse_x -= 1;
                    if fake_mouse_x < 0{
                        fake_mouse_x = 0;
                    }
                } else if c == 'l'{
                    fake_mouse_x += 1;
                } else if c == 'z'{
                    no_mouse_mode = !no_mouse_mode;
                }
            },

            Some(_input) => {  },
            None => ()
        }
        w.refresh();
    }
    endwin();
}
