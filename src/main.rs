use std::{env::{self, args, consts, current_dir}, ffi::OsStr, fs::{self}, path::PathBuf, process::exit, str::FromStr, time::Duration};
use walkdir::{self, WalkDir};

fn move_to_backup(file: walkdir::DirEntry, backup_directory: &PathBuf)
{
    let current_dir_length = current_dir().unwrap().to_string_lossy()
    .len();
    let path_length = file.path().to_string_lossy()
    .len();
    let filename_length = file.file_name().to_string_lossy()
    .len();
    
    let copy_result = fs::copy(file.clone().into_path(), format!("{}\\{}", backup_directory.to_string_lossy(), file.path().to_str().unwrap()[current_dir_length..].to_string()));
    if copy_result
    .is_err()
    {                    
        fs::create_dir_all(format!("{}\\{}",backup_directory.to_string_lossy(), (file.path().to_str().unwrap()[current_dir_length..path_length - filename_length].to_string())))
        .expect("Failed to create directories. Check permissions?");
        move_to_backup(file, backup_directory);
    }
    //if copy fails, attempt to make folder/folders, then call function again.
}

fn print_help_text(backup_directory: &PathBuf)
{
    println!("
    Backupr:\n
    -p\t\tPath to folder to place backed up files. Defaults to (OS Home Dir)/backups. or in your case, ({})\n\n
    -t\t\tTime in (seconds/hours/days/months/years) between file modification to backup.\tExample: 24d for 24 days. 24h for 24 hours. 1m for 1 month. 120s for 2 minutes.\n\n
    -ext and -ext-neg\t\tAllows you to only backup a certain file extension, or ignore a certain file extension in your backups.\tMay allow for you to run multiple of this program with different parameters, e.g ignoring .mp4, and running another process of backupr to only capture mp4s, and set a time of 3 days since modification.", &backup_directory.to_string_lossy());
}

fn time_parse (user_input: &String, time_format: &str, seconds: i64) -> i64
{
    return i64::from_str(&user_input.as_str()[..user_input.find(&time_format).unwrap()].to_string()).expect("Could not parse a number from your \"-t\" input. Did you enter it correctly?") * seconds;
}

fn parse_args(backup_directory: &mut PathBuf, time_to_check: &mut Duration, extension: &mut Vec<String>, ext_neg: &mut bool)
{

    let args: Vec<String> = args().collect();
    let arg2: Option<String> = if args.len() > 0
    {
        args.get(1).cloned()
    }
    else
    {
        None
    };

    if args.len() == 1
    {
        println!("No arguments provided, assuming defaults.\nDirectory: {}", backup_directory.to_string_lossy());
    }
    else if arg2.is_some() && ((arg2.as_ref().unwrap().eq_ignore_ascii_case("-help") || arg2.as_ref().unwrap().eq_ignore_ascii_case("-h") || arg2.as_ref().unwrap().eq_ignore_ascii_case("help")))
    {
        print_help_text(&backup_directory);
        exit(0);
    }

    drop(arg2);
    //memory conservation I hope? Won't do much but in this ram market ill take it.
    
        for (i,arg) in args.iter().enumerate()
        {
            if arg.trim().eq("-p") 
            {
                let next = args.get(i+1).expect("Could not get path after \"-p\" argument. Did you enter one?");

                if next.contains("\\")
                {
                    *backup_directory = PathBuf::from_str(&next.replace("\\", "/")).unwrap();
                }
                else if next.contains("/")
                {
                    *backup_directory = PathBuf::from_str(next).unwrap();
                }
                else
                {
                    println!("Path does not seem to contain any \"/'s\" or \"\\\"'s. Is this path valid?");
                }
            }
            else if arg.trim().eq("-t")
            {
                let next = args.get(i+1).expect("Could not get number after \"-t\" argument. Did you enter one?");

                let mut number: Option<i64> = None;

                if next.find("s").is_some()
                {
                    number = Some(time_parse(next, "s", 1));
                }
                else if next.find("h").is_some()
                {
                    number = Some(time_parse(next, "h", 3600));
                }
                else if next.find("d").is_some()
                {
                    number = Some(time_parse(next, "d", 86400));
                }
                else if next.find("m").is_some()
                {
                    number = Some(time_parse(next, "m", 2592000));
                }
                else if next.find("m").is_some()
                {
                    number = Some(time_parse(next, "y", 31556952));
                }

                if number.is_some()
                {
                    *time_to_check = std::time::Duration::from_secs(number.unwrap().try_into().unwrap());
                }
            }
            else if arg.trim().eq("-ext")
            {
                let next = args.get(i+1).expect("Could not get string after \"-ext\" argument. Did you enter one?");

                extension.push(next.trim().to_string());

            }
            else if arg.trim().eq("-ext-neg")
            {
                let next = args.get(i+1).expect("Could not get string after \"-ext\" argument. Did you enter one?");

                extension.push(next.trim().to_string());

                *ext_neg = true;

            }   
        }
}
fn main() 
{
    let folder_path = env::current_dir().unwrap();
    let mut time_to_check = std::time::Duration::from_secs(u64::MAX);
    // i dont really like this, but ill fix it later.
    let mut extension = Vec::<String>::new();
    let mut ext_neg = false;
    let mut backup_directory = 
    if consts::OS.eq("windows") 
    {
        PathBuf::from_str(&format!("{}\\backups{}", env::home_dir().unwrap().to_str().unwrap(), current_dir().unwrap().to_str().unwrap()[current_dir().unwrap().as_path().to_str().unwrap().rfind("\\").unwrap()..].to_string()))
    }
    else
    {
        PathBuf::from_str(&format!("{}/backups/{}", env::home_dir().unwrap().to_str().unwrap(), current_dir().unwrap().to_str().unwrap()[current_dir().unwrap().as_path().to_str().unwrap().rfind("/").unwrap()..].to_string()))
    }.expect("Failed to parse current folder path for default directory.");
    
    
    parse_args(&mut backup_directory, &mut time_to_check, &mut extension, &mut ext_neg);
    //this is about my second time working with command line args, if you're one to read code, don't study this part!
    //any ideas on how to improve it are greatly welcome :D


    for file in WalkDir::new(folder_path)
    {
        let file_path = file.as_ref().unwrap().path();
        let file_modified = file.as_ref().unwrap().metadata().unwrap().modified().unwrap();

        //if file.metadata().unwrap().accessed().unwrap()
        if file.as_ref().unwrap().file_type()
        .is_file() 
        {

            //if the extensions vec has values, executes this to check against them before backing up.
            if extension.is_empty()
            {
                if file_modified
                .elapsed().unwrap() > time_to_check
                {
                    break;
                }
                else
                {
                    move_to_backup(file.unwrap(), &backup_directory);
                }
            }
            else if ext_neg == false
            {
                for ext in &extension
                {
                    if file_path
                    .extension().unwrap().eq(OsStr::new(ext))
                    {
                        if file_modified
                        .elapsed().unwrap() > time_to_check
                        {
                            break;
                        }
                        else if file_modified
                        .elapsed().unwrap() < time_to_check 
                        {
                            move_to_backup(file.unwrap(), &backup_directory);
                            break;
                        }
                    }
                }
            }
            else if ext_neg == true
            {
                for ext in &extension
                {
                    if !file_path
                    .extension().unwrap().eq(OsStr::new(ext))
                    {
                        if file_modified
                        .elapsed().unwrap() > time_to_check
                        {
                            break;
                        }
                        else if file_modified
                        .elapsed().unwrap() < time_to_check 
                        {
                            move_to_backup(file.unwrap(), &backup_directory);
                            break;
                        }
                    }
                }
            }
        }
    }
}
