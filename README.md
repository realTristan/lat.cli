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


Quick Import (github_user)/(repo name):
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
- To make your own import, create a repository and put your .sty file inside. 
- Do not put the .sty file in any folders. 
- Do not have more than ONE .sty file inside the repo.
