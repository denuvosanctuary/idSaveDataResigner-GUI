> [!IMPORTANT]
> **This software is free and open source. If someone asks you to pay for it, it's likely a scam.**

# 🆔 idSaveDataResigner-GUI - What is it :interrobang:

A user-friendly GUI wrapper for idSaveDataResigner that allows you to **encrypt, decrypt, and resign SaveData files** from various games running on idTech Engine versions 7/8.

## Features
- **Main Tab**: Easy-to-use interface for Encrypt, Decrypt, and Resign operations.
- **Settings Tab**: Configure your desired output folder.
- **Batch Processing**: Processes all files in the selected folder.
- **Automatic Output Organization**: Creates folders with suffixes based on the operation mode.

## Supported titles
| Game Title                         | Steam App ID  |
|------------------------------------|---------------|
| DOOM Eternal                       | 782330        |
| Indiana Jones and the Great Circle | 2677660       |
| DOOM The Dark Ages                 | 3017860       |

## Supported platforms
It only supports Steam versions of the games.

# 🤔 Is it safe?
The short answer is: **No.** 
> [!CAUTION]
> If you unreasonably edit your SaveData files, you risk corrupting them or getting banned from playing online. In both cases, you will lose your progress.

> [!IMPORTANT]
> Always create a backup of any files before editing them.

> [!IMPORTANT]
> Disable the Steam Cloud before you replace any SaveData files.

You've been warned. Now that you fully understand the possible consequences, you may proceed to the next chapter.

# :scroll: How to use this tool

## Installation
1. Download the latest release from the releases page
2. Put the file to a folder of your choice
3. Run `resigner.exe`

## Usage

### Main Tab
1. **Select Operation**: Choose between Encrypt, Decrypt, or Resign
2. **Select Input Folder**: Browse and select the folder containing your SaveData files
3. **Game Code**: Select the appropriate game code for your save files
4. **User IDs**: 
   - For **Decrypt/Encrypt**: Enter your Steam ID (Old/Save SteamID)
   - For **Resign**: Enter both the original Steam ID and the target Steam ID
5. **Process**: Click the process button to start the operation

- It also adds an "INFO.txt" file inside your processed showcasing what you just did.

### Settings Tab
- **Output Folder**: Set your preferred output directory
- The processed files will be saved in this location with the same input folder name plus a suffix indicating the operation performed

### Important Notes
- **You must know your Old (Save) SteamID** as it's required for decryption
- **Transfer can be done to any SteamID** when resigning
- Processed files are organized automatically with descriptive folder names

> [!TIP]
> You can use the SteamDB calculator at [steamdb.info](https://steamdb.info/calculator/) to find your 64-bit SteamID.

## Building from Source
```bash
# Clone the repository
git clone https://github.com/denuvosanctuary/idSaveDataResigner-GUI.git
cd idSaveDataResigner-GUI

# Build the project
cargo build --release

# Run the application
cargo run
```

## Requirements
- A Functioning brain

# :heart: Credits
This GUI is built upon the work of [mi5hmash](https://github.com/mi5hmash/) and their [idSaveDataResigner](https://github.com/mi5hmash/idSaveDataResigner/) project. All core logic and SaveData processing functionality is credited to their original implementation.
