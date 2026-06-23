# Backupr
### No AI was used in the making of this.
Something I built to help me learn Rust to help backup files for my Minecraft Server, will also let you back up (hopefully) anything else!

Help Text:

    Backupr:
    -p    Path to folder to place backed up files. Defaults to (OS Home Dir)/backups.

    -t    Time in (seconds/hours/days/months/years) between file modification to backup.  Example: 24d for 24 days. 24h for 24 hours. 1m for 1 month. 120s for 2 minutes.
    
    -ext & -ext-neg    Allows you to only backup a certain file extension, or ignore a certain file extension in your backups.\tMay allow for you to run multiple of this program with different parameters, 
    e.g ignoring .mp4, and running another process of backupr to only capture mp4s, and set a time of 3 days since modification.
    pass \"-no-extension\" into -ext args for files with no extension)

Inspiration for most of these features ws not backing up old .mca regions, and only the new ones, as chunk pregeneration is pretty large. Though can be used for just about anything with the handy task scheduler.

Should have everything you need and nothing you don't.
