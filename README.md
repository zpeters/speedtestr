[![Build Status](https://travis-ci.org/zpeters/speedtestr.svg?branch=master)](https://travis-ci.org/zpeters/speedtestr)
[![Build status](https://ci.appveyor.com/api/projects/status/p0qq7rtmg7u3kwxl?svg=true)](https://ci.appveyor.com/project/zpeters/speedtestr)
![GitHub issues](https://img.shields.io/github/issues-raw/zpeters/speedtestr.svg)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# Speedtestr

Speedtestr is an opensource project to attempt to reverse engineer parts of the publically available speedtest.net system in order to create an unofficial command-line client.

The advantage of using the speedtest infrastructure is that as the end-user there is nothing to install, setup or maintain.  You just need to run the speedtestr client app.

Since this is an unofficial client using *unsupported* methods to test speeds it is possible this **completely innaccurate** and there is nothing I can do about that.  It is very possible that at any moment the *tricks* that are used to get speed test results could go away so *DO NOT DEPEND ON THIS FOR ANYTHING MISSION CRITICAL!!!*

This is just a fun skill-building exercise

# Notes
- Original reverse engineering of the tcp protocol - https://gist.github.com/sdstrowes/411fca9d900a846a704f68547941eb97
