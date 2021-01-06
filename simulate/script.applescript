on run
  pasteText()
end run

on pasteText()
    set inputfile to POSIX file "/Users/RajInMacLand/Exercism/rust/simulate/main.rs"
    try
      set getClip to the clipboard as text
    on error
      set getClip to " "
    end try
    try
      tell application "Finder"
        open file inputfile
      end tell
    on error
      display dialog ("Error. Couldn't delete files") buttons {"OK"}
    end try
    delay 1
    tell application "Electron" to activate
    delay 1
    tell application "System Events"
      repeat with i from 1 to count characters of getClip
        keystroke (character i of getClip)
        delay 0.1
      end repeat
    end tell
    tell application "System Events"
      keystroke "f" using (shift down, option down)
    end tell
end pasteText


