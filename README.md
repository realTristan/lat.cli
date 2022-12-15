# lat.cli ![Stars](https://img.shields.io/github/stars/realTristan/lat.cli?color=brightgreen) ![Watchers](https://img.shields.io/github/watchers/realTristan/lat.cli?label=Watchers)
![LaTeX_logo](https://user-images.githubusercontent.com/75189508/207660487-afff32e3-9ac2-474e-b3b2-36572537e272.png)

# About
- lat.cli is a fast and easy-to-use CLI Tool for importing .sty files from github into your project.
- lat.cli makes importing simple. No Urls. Easy Imports. Customizable Shortcuts.
- lat.cli was built with Rust to ensure minimal memory usage and maximum speed.

# Install
```
MacOS:
  $ curl "https://github.com/realTristan/lat.cli/blob/main/lat?raw=true" -o /usr/local/bin/lat
  
Windows:
  $ mkdir C:\lat.cli
  $ curl "https://github.com/realTristan/lat.cli/blob/main/lat.exe?raw=true" -o C:\lat.cli\lat.exe
  $ Add "C:\lat.cli" to Environment Variables
```

# Example Import
```
$ cd your_latex_directory

Import Using the Repository:
  $ lat -i https://github.com/realTristan/realtristan.sty


Quick Import (github_user)/(repo and file name):
  $ lat -i realTristan/realtristan.sty


Import Using the File Url:
  $ lat -i https://github.com/realTristan/realtristan.sty/blob/main/realtristan.sty
```

# Example Shortcuts
```
$ lat -short -new rt realTristan/realtristan.sty
$ lat -i rt

$ lat -short -list
$ lat -short -remove rt
$ lat -short -empty
```

# Making your own import
The repository name must be the same name as the .sty file name.

<img width="323" alt="Screen Shot 2022-12-14 at 11 57 59 AM" src="https://user-images.githubusercontent.com/75189508/207659388-222be577-aeee-43f3-93e4-13fa8b4a0995.png">
<img width="358" alt="Screen Shot 2022-12-14 at 11 58 55 AM" src="https://user-images.githubusercontent.com/75189508/207659400-18a60ed8-715d-44fb-9b21-1bb8e79a759a.png">
