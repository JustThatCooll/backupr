# Backupr
### No AI was used in the making of this.
Something I built to help me learn Rust to help backup files for my Minecraft Server, will also let you back up (hopefully) anything else!

Help Text:

    Backupr: 
    -p  Path to folder to place backed up files. Defaults to (OS Home Dir)/backups. or in your case, ({})  
    -bp  Path to folder to backup, optional, will back up current directory as fallback.  
    -t  Time in (seconds/hours/days/months/years) between file modification to backup. Example: 24d for 24 days. 24h for 24 hours. 1m for 1 month. 120s for 2 minutes.  
    -ext and -ext-neg  Allows you to only backup a certain file extension, or ignore a certain file extension in your backups. May allow for you to run multiple of this program with different parameters, e.g ignoring .mp4, and running another process of backupr to only capture mp4s, and set a time of 3 days since modification. 
    pass  -no-extension  into -ext args for files with no extension

    Filename modifiers:
    Modifies file fully modularly, prebuilt date-time for US and EU timestamps, or make your own.

    -zoneUTC -zoneLocal, -zone, -date, -dateUS, -dateEU, -day, -month, -year, -day-name, -month-name, -time, -time-ms -time-ns, -seperator -arg-seperator
    Modifiers:
        -zoneUTC uses Coordinated Universal Time for the timezone, -zoneLocal will use your local timezone, -zone will print your local timezone offset.
        -seperator will change what is used to seperate tokens in text, if time was 12 o'clock, would be 12(seperator)00 e.g 12:00, 12.00.
        -arg-seperator will change what is used to seperate each different tokens. Default is  . 
    Text:
        -date will add the date in ISO 8601 format, -dateUS will be current date in MDY fashion, and -dateEU in European fashion (YMD).
        -day -month and -year, will print day, month, and year in format numerical. (Unless your from beyond 10,000), then so on and so forth.
        -day-name and -month-name will print the name of the day, such as  Tuesday  and  February .
        -time, -time-ms and -time-ns will place the current time in  00(seperator)00  and  00(seperator)00(seperator)00  format respectively.
    All of the modifier arguments will take effect on the parameters after them, allowing for multiple uses of the seperator function to change between args.

Inspiration for most of these features ws not backing up old .mca regions, and only the new ones, as chunk pregeneration is pretty large. Though can be used for just about anything with the handy task scheduler.

Should have everything you need and nothing you don't.
