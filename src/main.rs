use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Clone)]
enum FileType {
    Text,
    Binary,
}
#[derive(Clone)]
struct File {
    name: String,
    content: Vec<u8>, // max 1000 bytes, rest of the file truncated
    creation_time: u64,
    type_: FileType,
}

impl File {
    fn new(name: String, mut content: Vec<u8>, file_type: FileType) -> Self {
        let creation_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        content.truncate(1000);
        Self {
            name,
            content,
            creation_time,
            type_: file_type,
        }
    }
}
#[derive(Clone)]
struct Dir {
    name: String,
    creation_time: u64,
    children: Vec<Node>,
}

impl Dir {
    fn new(name: String, children: Vec<Node>) -> Self {
        let creation_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            name: name,
            creation_time: creation_time,
            children: children,
        }
    }
}
#[derive(Clone)]
enum Node {
    File(File),
    Dir(Dir),
}

struct FileSystem {
    root: Dir,
}

impl FileSystem {
    fn new() -> Self {
        let name = "root".to_string();
        Self {
            root: Dir::new(name, Vec::new()),
        }
    }

    fn print(&self) {
        let root = &self.root;
        println!("-----------------------------------");
        //print_recursive(root);

        print_tree(&Node::Dir(self.root.clone()), 0);
    }
    fn from_dir(path: &str) -> Self {
        let mut current_root = Dir::new(path.to_string(), Vec::new());

        visita_in_profondita(&mut current_root, path);

        Self { root: current_root }
    }

    fn make_dir(&mut self, path: &str) {
        let mut current_dir = &mut self.root;
        let path_v: Vec<&str> = path.split("/").collect();

        search_and_create(current_dir, &path_v, 1);
    }

    fn rm_dir(&mut self, path: &str) {
        let mut current_dir = &mut self.root;

        let path_v: Vec<&str> = path.split("/").collect();

        search_and_remove(current_dir, &path_v, 1);
    }
}

fn print_tree(node: &Node, depth: usize) {
    match node {
        Node::File(file) => {
            println!("{:indent$}File: {}", "", file.name, indent = depth * 4);
        }
        Node::Dir(dir) => {
            println!("{:indent$}Dir: {}", "", dir.name, indent = depth * 4);
            for child in &dir.children {
                print_tree(child, depth + 1);
            }
        }
    }
}

fn visita_in_profondita(current_root: &mut Dir, path: &str) {
    let paths = fs::read_dir(path).unwrap();
    for p in paths {
        match p {
            Ok(entry) => {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(created) = metadata.created() {
                        if let Ok(duration) = created.duration_since(UNIX_EPOCH) {
                            let name = entry.file_name();
                            let creation_time = duration.as_secs();
                            let new_path = entry.path();
                            if metadata.is_file() {
                                let contents = fs::read(entry.path()).unwrap();
                                if contents.contains(&0u8) {
                                    //is binary if contains null bytes
                                    let found = File::new(
                                        name.into_string().unwrap(),
                                        contents,
                                        FileType::Binary,
                                    );

                                    current_root.children.push(Node::File(found));
                                } else {
                                    let found = File::new(
                                        name.into_string().unwrap(),
                                        contents,
                                        FileType::Text,
                                    );
                                    //println!("added textual file {} to {}", found.name,current_root.name);
                                    current_root.children.push(Node::File(found));
                                }
                            } else {
                                // è un directory
                                let mut found = Dir::new(name.into_string().unwrap(), Vec::new());

                                visita_in_profondita(&mut found, new_path.to_str().unwrap());
                                //println!("added directory {} to {}", found.name,current_root.name);
                                current_root.children.push(Node::Dir(found));
                            }
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        };
    }
}

fn search_and_create(current_dir: &mut Dir, path_v: &Vec<&str>, index: usize) {
    if index >= path_v.len() {
        return;
    }
    let target_dir_name = path_v[index];
    for node in &mut current_dir.children {
        if let Node::Dir(dir) = node {
            if dir.name == target_dir_name.to_string() {
                search_and_create(dir, &path_v, index + 1);
                return;
            }
        }
    }
    if index == path_v.len() - 1 {
        let new_dir = Dir::new(target_dir_name.to_string(), Vec::new());
        current_dir.children.push(Node::Dir(new_dir));
    }
}

fn search_and_remove(current_dir: &mut Dir, path_v: &Vec<&str>, index: usize) -> bool {
    if index >= path_v.len() {
        if current_dir.children.len() == 0 {
            return true;
        }
        return false;
    }
    let target_dir_name = path_v[index];
    let mut index_to_remove = None;
    for (i, node) in current_dir.children.iter_mut().enumerate() {
        if let Node::Dir(dir) = node {
            if dir.name == target_dir_name.to_string() {
                if search_and_remove( dir, &path_v, index + 1) {
                    index_to_remove = Some(i);
                    if let Some(index) = index_to_remove {
                        println!("removing {}",dir.name);
                        current_dir.children.remove(index);
                    }
                }
                
                return false;
            }
        }
    }
    false
}
fn main() {
    let mut file_system = FileSystem::new();

    file_system = FileSystem::from_dir("./src");

    file_system.print();
    //la root di file_system ora è src
    file_system.make_dir("/libc/created");

    file_system.print();
    

    file_system.rm_dir("/libc/created");
    file_system.print();
}
