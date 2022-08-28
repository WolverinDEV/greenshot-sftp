# Command line tool for uploading greenshot commands via sftp

## CLI Interface
```text
greenshot_sftp 0.1.0

USAGE:
    greenshot_sftp.exe [OPTIONS] --server-url <SERVER_URL> --user <USER> --private-key <PRIVATE_KEY> --target-directory <TARGET_DIRECTORY> --source <SOURCE>

OPTIONS:
        --clipboard-url <CLIPBOARD_URL>


    -h, --help
            Print help information

    -i, --source <SOURCE>
            Image source path. When used with getgreenshot use "{0}"

    -k, --private-key <PRIVATE_KEY>


    -p, --private-key-passphrase <PRIVATE_KEY_PASSPHRASE>


    -s, --server-url <SERVER_URL>


    -t, --target-directory <TARGET_DIRECTORY>


    -u, --user <USER>


    -V, --version
            Print version information
```
Note:
In order to view the command line output comment out `#![windows_subsystem = "windows"]`.  
When using with Greenshot it's annoying to have a window popping up.  

Example arguments:  
```
-s img.did.science:22 -u root -k "C:/Users/WolverinDEV/.ssh/id_rsa" -t "/var/www/img.mcgalaxy" --clipboard-url "https://img.did.science/[file_name]" -i "{0}"
```