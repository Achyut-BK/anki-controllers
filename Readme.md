
# Plugin to use a generic Controller with Anki
Goal: Use a Controller with anki to Review Cards with ease
### Compatibility
- Any Controller with this generic Layout should work
![Generic Controller Layout](https://raw.githubusercontent.com/Achyut-BK/anki-controllers/145d866684e77496c420dda689916b44cc4c2b97/controller_default.svg)
- So far, The DualShock 4 has been confirmed to work. XBox Controllers and Other Dualshocks should also be compatible. If any other controllers work, please create an issue so that I can add it to the compatibility list.
- This Program has been tested with Linux & Windows 10, but the Windows exe should also be compatible with Windows 11. Mac OSX should work if you build it yourself.
- Note, XBox Controllers work on windows out of the box due to Microsoft being inconsistent with APIs. You need to find a way to convert DirectInput events to XInput events.
- For a Dualshock 4 or 5, installing [DS4Windows](https://www.pcgamer.com/how-to-use-a-ps4-controller-on-pc/#section-ds4windows/) and having DS4Windows.exe open while running anki_controllers will suffice.

### Control Scheme
![Controller Button Map](https://raw.githubusercontent.com/Achyut-BK/anki-controllers/145d866684e77496c420dda689916b44cc4c2b97/controller_map.svg)
### How to Install
1. Install the [AnkiConnect Addon](https://ankiweb.net/shared/info/2055492159)
2. Download the executable for your OS from [our releases page](https://github.com/Achyut-BK/anki-controllers)
3. Releases are available for :
 - Linux
 - Windows
### How to run
1. Open Anki, and double click on Anki_Controllers.exe (Windows), or Anki_Controllers (Linux)
2. Press the **Start** button on your controller
### Building the repository yourself
- ```git clone https://github.com/Achyut-BK/anki-controllers.git```
- ``` cd anki-controllers```
- ```cargo build --release```