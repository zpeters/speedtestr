Appveyor (Windows)
[![Build status](https://ci.appveyor.com/api/projects/status/fxunefx1h5o1n3s8?svg=true)](https://ci.appveyor.com/project/zpeters/speedtestr)
Travis CI (Linux)
[![Build Status](https://travis-ci.org/zpeters/speedtestr.svg?branch=master)](https://travis-ci.org/zpeters/speedtestr)
[![Gitter](https://img.shields.io/gitter/room/zpeters/speedtestr.svg)
 [![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# TODO

## General
- general refactor of code.  This is embarrassing. 😕 I'm still learning rust and i'm getting this out as fast as possible and learning correct techniques as i go.
- figure out what the UI should be
- once mechanics are completed, implement actual testing (ie sustained uplaod, download, etc)
- better error handling on bad server id
- makefiles for cross compile or use travis/appveyor for xplatform builds
- doc
- testing

## lib.rs
- faster random byte generation for upload

## cleanup / refactor
- clean up main.rs
- clean up lib.rs


# Notes
- Original reverse engineering of the tcp protocol - https://gist.github.com/sdstrowes/411fca9d900a846a704f68547941eb97
