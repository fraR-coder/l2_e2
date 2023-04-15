use std::{fs, path::Path};

enum FileType {
    Text,
    Binary,
}
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


   


    

   /*  fn from_dir(path: &str) -> Self {
        let root_name = Path::new(path)
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
        let root = Dir::new(root_name, Vec::new());
        let mut file_system = FileSystem { root };
        file_system.copy_dir(path);
        file_system
    }

    fn copy_dir(&mut self, path: &str) {
        for entry in fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let entry_path = entry.path();
            let entry_name = entry.file_name().into_string().unwrap();
            if entry_path.is_dir() {
                let mut new_dir = Dir::new(entry_name, Vec::new());
                self.copy_dir(entry_path.to_str().unwrap());
                self.root.children.push(Node::Dir(new_dir));
            } else {
                let content = fs::read(entry_path).unwrap();
                let new_file = File::new(entry_name, content, FileType::Binary);
                self.root.children.push(Node::File(new_file));
            }
        }
    }
    */

    

    /*fn find_in_dir<'a>(&self, src_dir: &'a mut Dir, target_dir_name: &str) -> Option<&'a mut Dir> {
        let children_list = &mut src_dir.children;
        for node in children_list {
            if let Node::Dir(dir) = node {
                if dir.name == target_dir_name.to_string() {
                    return Some(dir);
                }
            }
        }
        None
    }
    */

    fn make_dir(&mut self, path: &str) {
        let mut current_dir = &mut self.root;
        let path_v: Vec<&str> = path.split("/").collect();

        search_and_create( current_dir,&path_v,1);
       
    }
}

fn search_and_create(current_dir: &mut Dir, path_v: &Vec<&str>, index:usize) {
    if index>= path_v.len() {
        return;
    }
    let target_dir_name=path_v[index];
    for node in &mut current_dir.children {
       
        if let Node::Dir(dir) = node {
            if dir.name == target_dir_name.to_string() {
               println!("found dir {} ", dir.name);
                search_and_create( dir, &path_v,index+1);
                return;
            }
            
        }
        
    }
    if index==path_v.len()-1{
    println!("creating directory {}", target_dir_name);
    let new_dir=Dir::new(target_dir_name.to_string(),Vec::new());
    current_dir.children.push(Node::Dir(new_dir));
    }

}
fn main() {
    let mut file_system = FileSystem::new();
    
    file_system.make_dir("/a");
    file_system.make_dir("/c");
    file_system.make_dir("/a/c");
    file_system.make_dir("/a/b/d");
    file_system.make_dir("/a/b/d/e");
   

    
    
}
