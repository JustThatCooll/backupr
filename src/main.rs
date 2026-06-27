use std::{env::{self, args, consts, current_dir}, ffi::OsStr, fs::{self}, path::{self, PathBuf}, process::exit, str::FromStr, time::Duration};
use time::OffsetDateTime;
use walkdir::{self, WalkDir};

fn move_to_backup(file: walkdir::DirEntry, backup_directory: &PathBuf, folder_path: &path::PathBuf)
{
    let current_dir_length = folder_path.to_str().unwrap()
    .len();
    let path_length = file.path().to_string_lossy()
    .len();
    let filename_length = file.file_name().to_string_lossy()
    .len();
    
    let copy_result = fs::copy(file.clone().into_path(), format!("{}\\{}", backup_directory.to_string_lossy(), file.path().to_str().unwrap()[current_dir_length..].to_string()));
    if copy_result
    .is_err()
    {                    
        fs::create_dir_all(format!("{}/{}",backup_directory.to_string_lossy(), (file.path().to_string_lossy()[current_dir_length..path_length - filename_length].to_string())))
        .expect("Failed to create directories. Check permissions?");
        move_to_backup(file, backup_directory, folder_path);
    }
    //if copy fails, attempt to make folder/folders, then call function again.
}

fn print_help_text(backup_directory: &PathBuf)
{
    println!("
    Backupr:\n
    -p\t\tPath to folder to place backed up files. Defaults to (OS Home Dir)/backups. or in your case, ({})\n\n
    -bp\t\tPath to folder to backup, optional, will back up current directory as fallback.\n\n
    -t\t\tTime in (seconds/hours/days/months/years) between file modification to backup.\tExample: 24d for 24 days. 24h for 24 hours. 1m for 1 month. 120s for 2 minutes.\n\n
    -ext and -ext-neg\t\tAllows you to only backup a certain file extension, or ignore a certain file extension in your backups.\tMay allow for you to run multiple of this program with different parameters, e.g ignoring .mp4, and running another process of backupr to only capture mp4s, and set a time of 3 days since modification.\n
    (pass \"-no-extension\" into -ext args for files with no extension

    Filename modifiers:
    Modifies file fully modularly, prebuilt date-time for US and EU timestamps, or make your own.

    -zoneUTC -zoneLocal, -zone, -date, -dateUS, -dateEU, -day, -month, -year, -day-name, -month-name, -time, -time-ms -time-ns, -seperator -arg-seperator
    Modifiers:
        -zoneUTC uses Coordinated Universal Time for the timezone, -zoneLocal will use your local timezone, -zone will print your local timezone offset.
        -seperator will change what is used to seperate tokens in text, if time was 12 o'clock, would be 12(seperator)00 e.g 12:00, 12.00.
        -arg-seperator will change what is used to seperate each different tokens. Default is \".\"
    Text:
        -date will add the date in ISO 8601 format, -dateUS will be current date in MDY fashion, and -dateEU in European fashion (YMD).
        -day -month and -year, will print day, month, and year in format numerical. (Unless your from beyond 10,000), then so on and so forth.
        -day-name and -month-name will print the name of the day, such as \"Tuesday\" and \"February\".
        -time, -time-ms and -time-ns will place the current time in \"00(seperator)00\" and \"00(seperator)00(seperator)00\" format respectively.
    All of the modifier arguments will take effect on the parameters after them, allowing for multiple uses of the seperator function to change between args.", &backup_directory.to_string_lossy());
}

fn time_parse (user_input: &String, time_format: &str, seconds: i64) -> i64
{
    return i64::from_str(&user_input.as_str()[..user_input.find(&time_format).unwrap()].to_string()).expect("Could not parse a number from your \"-t\" input. Did you enter it correctly?") * seconds;
}

fn month_to_number (current_time: OffsetDateTime) -> String
{
    let month_array = vec!["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

    let mut month_number: Option<String> = None;

    for (i, month) in month_array.iter().enumerate()
    {
        if month.trim() == (&current_time.month().to_string())
        {
            month_number = Some((i + 1).to_string());
            break;
        }
    }
    if month_number.is_none()
    {                  
        println!("Could not find month in -month argument.");
        exit(1);
    }

    return month_number.unwrap();
}

fn parse_args(backup_directory: &mut PathBuf, time_to_check: &mut Duration, extension: &mut Vec<String>, ext_neg: &mut bool, folder_path: &mut path::PathBuf)
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
        println!("No arguments provided, assuming defaults. Use -help to see arguments.\nDirectory: {}", backup_directory.to_string_lossy());
    }
    else if arg2.is_some() && ((arg2.as_ref().unwrap().eq_ignore_ascii_case("-help") || arg2.as_ref().unwrap().eq_ignore_ascii_case("-h") || arg2.as_ref().unwrap().eq_ignore_ascii_case("help")))
    {
        print_help_text(&backup_directory);
        exit(0);
    }


    drop(arg2);
    //memory conservation I hope? Won't do much but in this ram market ill take it.

    //I would like to figure out a way to make this only initialize once one of the time options is selected, but I think the performance loss is super minimal for now.
    let mut current_time = time::OffsetDateTime::now_utc();
    let mut time_added = false;
    let mut arg_seperator = "-";
    let mut seperator = ".";

        for (i,arg) in args.iter().enumerate()
        {
            let arg_trim = arg.trim();
            if arg_trim.eq("-p") 
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
                    println!("Path does not seem to contain any \"/'s\" or \"\\\"'s. Is this path valid? Path: {}", next);
                }
            }
            else if arg_trim.eq("-bp")
            {
                let next = args.get(i+1).expect("Could not get path after \"-bp\" argument. Did you enter one?");

                if next.contains("\\")
                {
                    *folder_path = PathBuf::from_str(&next.replace("\\", "/")).unwrap();
                }
                else if next.contains("/")
                {
                    *folder_path = PathBuf::from_str(next).unwrap();
                }
                else
                {
                    println!("Path does not seem to contain any \"/'s\" or \"\\\"'s. Is this path valid? Path: {}", next);
                }
            }
            else if arg_trim.eq("-date")
            {   
                if time_added == false
                {         
                    time_added = true;   
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), current_time.date().to_string().replace(":", seperator))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.date().to_string().replace(":", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-dateUS")
            {   
                let month_number = month_to_number(current_time);

                if time_added == false
                {             
                    time_added = true;          
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}{seperator}{}{seperator}{}", month_number, current_time.day(), current_time.year()).replace(":", seperator))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), format!("{}{seperator}{}{seperator}{}", month_number, current_time.day(), current_time.year()).replace(":", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-dateEU")
            {
                let month_number = month_to_number(current_time);

                if time_added == false
                {             
                    time_added = true;          
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}{seperator}{}{seperator}{}", current_time.day(), month_number , current_time.year()).replace(":", seperator))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), format!("{}{seperator}{}{seperator}{}", current_time.day(), month_number, current_time.year()).replace(":", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-day-name")
            {                
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}", current_time.weekday()))).unwrap();
                }
                else if time_added == true
                {
                    
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.weekday())).unwrap();
                }
            }
            else if arg_trim.eq("-day")
            {                
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}", current_time.day()))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.day())).unwrap();
                }
            }
            //I cannot, for the life of me, figure out how in the world to get this to be a numerical value.
            //while I could make my own array and iterate over it for month values, which I very well may do, this is confusing me greatly.
            //I see in the docs about the month enum, and I've implemented the parsing method, yet I can't seem to figure out how to get it.
            else if arg_trim.eq("-month")
            {               

                let month_number = month_to_number(current_time);

                if time_added == false
                {                
                    time_added = true;       
                    
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), month_number)).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), month_number)).unwrap();
                }
            }
            
            else if arg_trim.eq("-month-name")
            {
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}", current_time.month()))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.month())).unwrap();
                }
            }
            else if arg_trim.eq("-year")
            {                
                if time_added == false
                {              
                    time_added = true;         
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}", current_time.year()))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.year())).unwrap();
                }
            }
            else if arg_trim.eq("-time")
            {                
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), format!("{}{seperator}{}", current_time.time().hour().to_string(), current_time.time().minute()))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.time().to_string().replace(":", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-time-ms")
            {                
                if time_added == false
                {              
                    time_added = true;         
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), current_time.millisecond().to_string().replace(":", seperator).replace(".", seperator))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.millisecond().to_string().replace(":", seperator).replace(".", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-time-ns")
            {                
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), current_time.nanosecond().to_string().replace(":", seperator).replace(".", seperator))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.nanosecond().to_string().replace(":", seperator).replace(".", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-month")
            {                
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), current_time.month())).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.month())).unwrap();
                }
            }
            else if arg_trim.eq("-zone")
            {                
                if time_added == false
                {                
                    time_added = true;       
                    *backup_directory = PathBuf::from_str(&format!("{}/{}", backup_directory.to_str().unwrap().to_string(), current_time.offset().to_string().replace(":", seperator))).unwrap();
                }
                else if time_added == true
                {
                    *backup_directory = PathBuf::from_str(&format!("{}{arg_seperator}{}", backup_directory.to_str().unwrap().to_string(), current_time.offset().to_string().replace(":", seperator))).unwrap();
                }
            }
            else if arg_trim.eq("-arg-seperator")
            {
                let next = args.get(i+1).expect("Could not get string after \"-arg-seperator\" argument. Did you enter one?");
                arg_seperator = next;
            }
            else if arg_trim.eq("-seperator")
            {
                let next = args.get(i+1).expect("Could not get string after \"-seperator\" argument. Did you enter one?");
                seperator = next
            }
            else if arg_trim.eq_ignore_ascii_case("-zoneUTC")
            {
                current_time = time::OffsetDateTime::now_utc();
            }
            else if arg_trim.eq_ignore_ascii_case("-zoneLocal")
            {
                current_time = time::OffsetDateTime::now_local().expect("Could not get local time.");
            }
            else if arg_trim.eq("-t")
            {
                let next = args.get(i+1).expect("Could not get number after \"-t\" argument. Did you enter one?");
                //improper error handling here, I should fix this, needs to check if there is a number in this argument as well before panicking.
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
            else if arg_trim.eq("-ext")
            {
                let next = args.get(i+1).expect("Could not get string after \"-ext\" argument. Did you enter one?");

                extension.push(next.trim().to_string().replace(".", ""));

            }
            else if arg_trim.eq("-ext-neg")
            {
                let next = args.get(i+1).expect("Could not get string after \"-ext\" argument. Did you enter one?");

                extension.push(next.trim().to_string().replace(".", ""));

                *ext_neg = true;

            }   
        }
}
fn main() 
{
    let mut folder_path = env::current_dir().unwrap();
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
    
    
    parse_args(&mut backup_directory, &mut time_to_check, &mut extension, &mut ext_neg, &mut folder_path);
    println!("{}", backup_directory.to_string_lossy());
    //this is about my second time working with command line args, if you're one to read code, don't study this part!
    //any ideas on how to improve it are greatly welcome :D


    for file in WalkDir::new(&folder_path)
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
                    move_to_backup(file.unwrap(), &backup_directory, &folder_path);
                }
            }
            else if ext_neg == false
            {
                for ext in &extension
                {
                    if file_path.extension().is_some()
                    {
                        if file_path
                        .extension().unwrap().eq(OsStr::new(ext))
                        {
                            if file_modified
                            .elapsed().unwrap() < time_to_check 
                            {
                                move_to_backup(file.unwrap(), &backup_directory, &folder_path);
                                break;
                            }
                        }
                    }
                    else if file_path.extension().is_none() && ext.eq_ignore_ascii_case("-no-extension")
                    {
                        if file_modified
                        .elapsed().unwrap() < time_to_check 
                        {
                            move_to_backup(file.unwrap(), &backup_directory, &folder_path);
                            break;
                        }
                    }
                }
            }
            else if ext_neg == true
            {
                for ext in &extension
                {
                    if file_path.extension().is_some()
                    {
                        if !file_path
                        .extension().unwrap().eq(OsStr::new(ext))
                        {
                            if file_modified
                            .elapsed().unwrap() < time_to_check 
                            {
                                move_to_backup(file.unwrap(), &backup_directory, &folder_path);
                                break;
                            }
                        }
                    }
                    //checks if "-no-extension" is a value in the extension array for handling files with no extension.
                    else if file_path.extension().is_none() && ext.eq_ignore_ascii_case("-no-extension")
                    {
                        if file_modified
                        .elapsed().unwrap() < time_to_check 
                        {
                            move_to_backup(file.unwrap(), &backup_directory, &folder_path);
                            break;
                        }
                    }
                }
            }
        }
    }
    println!("Backupr'd to {}", backup_directory.to_string_lossy());
}
