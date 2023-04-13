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
    fn new(name: String,  mut content: Vec<u8>, file_type: FileType) -> Self {
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

    


    fn make_dir(&mut self, path: &str) {
        let mut current_dir = &mut self.root;
        let path_v:Vec<&str>=path.split("/").collect();
        println!("{:?}", path_v);
        let mut found = 0;
    
        for dir_name in path_v {
            found = 0;
            for node in &mut current_dir.children {
               
                if let Node::Dir(dir) = node { //same check as above
                    
                    if (dir.name == dir_name) {
                        current_dir = dir;
                        found = 1;
                        break;
                    }
                }
            }
            if (found == 0) {
               let new_dir=Dir::new(dir_name.to_string(),  Vec::new());
               current_dir.children.push(Node::Dir(new_dir));

            }
        }

      
    }
}

fn main() {
    let file_system = FileSystem::new();
    //let f=FileSystem::from_dir("/a/b");
}
