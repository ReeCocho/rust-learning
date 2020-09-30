/// A file on disk.
struct File
{
    /// The name of the file.
    name : String,

    /// The size in bytes of the file.
    size : u32
}

impl File
{
    /// File constructor.
    /// `name` is the name of the new file on disk.
    /// `size` is the size in bytes of the file.
    fn new(name : String, size : u32) -> File
    {
        File 
        { 
            name : name, 
            size : size 
        }
    }
}

/// A directory on disk which contains files.
struct Directory
{
    /// The name of the directory.
    name : String,

    /// The files contained within the directory.
    files : Vec<File>,

    /// Other directories contained within this directory
    dirs : Vec<Directory>
}

impl Directory
{
    /// Directory constructor.
    /// `name` is the name of the directory on disk.
    fn new(name : String) -> Directory
    {
        Directory
        {
            name : name,
            files : Vec::<File>::new(),
            dirs : Vec::<Directory>::new()
        }
    }

    /// Adds a new file to the directory.
    /// `file` is the new file to add.
    fn add_file(&mut self, file : File)
    {
        self.files.push(file);
    }

    /// Adds a new directory to this directory.
    /// `dir` is the new directory to add.
    fn add_dir(&mut self, dir : Directory)
    {
        self.dirs.push(dir);
    }
}

fn main() 
{

}

/// Generates a random directory tree structure
fn generate_random_directory()
{
    // Toss a coin to see if we are going to generate or not
    // let valid = rand::thread_rng().next_u32();
}