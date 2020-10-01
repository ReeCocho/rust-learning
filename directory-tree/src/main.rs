use rand::Rng;

fn main() 
{
    let dir = generate_random_directory(4);
    print_dir(&dir, 0);
}

/// Recursively prints out the contents of a directory.
/// 
/// `dir` is the directory to print out.
/// 
/// `indent` is the total number of spaces used for indentation.
fn print_dir(dir : &Directory, indent : u32)
{
    // Create a string containing the needed number of spaces
    let mut spaces = String::with_capacity(indent as usize);
    for _ in 0..indent
    {
        spaces.push(' ');
    }

    // Print out the current directory
    println!("{}{}", spaces, dir.name);

    // Print out files
    for file in &dir.files
    {
        // Print out files
        println!("  {}{} (size {})", spaces, file.name, file.size);
    }

    // Print out sub directories
    for sub_dir in &dir.dirs
    {
        print_dir(sub_dir, indent + 2);
    }
}

/// Generates a random directory tree structure.
/// 
/// `level` is the maximum depth of the directory tree
fn generate_random_directory(level : u32) -> Directory
{
    // Generate random directory name
    let dir_id : i32 = rand::thread_rng().gen_range(1, 100);
    let dir_name = String::from("Directory ") + &dir_id.to_string();

    // Create directory
    let mut dir : Directory = Directory::new(dir_name);

    // 20% chance to end early
    if rand::thread_rng().gen_range(1, 5) != 1 
    {
        // If we're here, it's time to generate files and subdirectories
        
        // Generate 1 to 4 sub directories if we aren't the last level
        if level > 0
        {
            for _ in 0..rand::thread_rng().gen_range(1, 4)
            {
                dir.add_dir(generate_random_directory(level - 1));
            }
        }

        // Generate 1 to 6 files
        for _ in 0..rand::thread_rng().gen_range(1, 6)
        {
            // Generate random file name
            let file_id : i32 = rand::thread_rng().gen_range(1, 100);
            let file_name = String::from("File ") + &file_id.to_string();

            // Generate random file size
            let file_size : u32 = rand::thread_rng().gen_range(1024, 4096);

            // Create file
            let file : File = File::new(file_name, file_size);

            // Add file to directory
            dir.add_file(file);
        }
    }

    return dir;
}

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