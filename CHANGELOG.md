# Changelog

All notable changes to this project will be documented in this file.

## [4.1.0] - 2025-03-12

### Bug fixes

- Improve prayer time accuracy by rounding seconds to the nearest minute ([bc2678f](https://github.com/azzamsa/islam/commit/bc2678f2a825f6a0952b5e1ac8cac485da8458ec))

  This improves accuracy by using [jadwalsholat.org](https://www.jadwalsholat.org/) as a reference.

## [4.0.0] - 2024-08-07

### Features

- Port chrono to jiff ([e538ff2](https://github.com/azzamsa/islam/commit/e538ff254a55b517e4518eeb479413c5d5aa8382))

## [3.2.0] - 2023-09-03

### Bug fixes

- Next prayer after midnight should be `fajr` ([634b5d9](https://github.com/azzamsa/islam/commit/634b5d9c0d1b7036f790112882986547f4c07ee5))

## [3.1.0] - 2023-09-01

### Bug fixes

- Wrong remaining time after ishaa salah ([64b2b7d](https://github.com/azzamsa/islam/commit/64b2b7d58de634fc3053f73189e610d8578020b3))

## [3.0.0] - 2023-08-30

### Features

- Ability to specify custom salah time ([bb48646](https://github.com/azzamsa/islam/commit/bb48646041bd89128095d27c446bdce91a18b4dd))

### Bug fixes

- Wrong remaining time using custom salah time ([addc21a](https://github.com/azzamsa/islam/commit/addc21a7cbedc0ade0779928ece0c14cafa8f747))
- Custom salah time ([540c12a](https://github.com/azzamsa/islam/commit/540c12a8d0f54cef2ff1442fbfc0576f2d6b65a2))
- Wrong salah name after midnight ([4912ce1](https://github.com/azzamsa/islam/commit/4912ce15c3b32ce0f2c09a0e1937593e7069cd21))
- Don't use dummy value ([f141eea](https://github.com/azzamsa/islam/commit/f141eea1b9eede952bfec6f1c7e1aaf172b2f961))

## [2.0.0] - 2023-06-22

### Bug fixes

- Restructure ([eb33501](https://github.com/azzamsa/islam/commit/eb33501733580178938e689a3147983eb58e5b0a))
- Use chrono v0.4.26 ([b57aa45](https://github.com/azzamsa/islam/commit/b57aa45e1f6855d25c082cb1273a95ab06f3c633))
- Migrate back to chrono ([bc53fa5](https://github.com/azzamsa/islam/commit/bc53fa59123c0ae0ebea5e16f8be0655c8e821f6))

  Users are not able to use this library in multiple threads.

## [1.0.0] - 2023-01-22

### Bug fixes

- User doesn't need to pass UTC offset anymore ([e29c32e](https://github.com/azzamsa/islam/commit/e29c32e2bbea3b7eb62cf134ba58a247f851b8c5))
  - **BREAKING!** ⚠️ : `time` has a feature to get the UTC offset from the user's
    system. Passing the offset to the app is redundant.

## [0.1.5] - 2022-05-18

### Bug fixes

- They are not executables ([31bf1fd](https://github.com/azzamsa/islam/commit/31bf1fd982fdf4aae30e1a94dd1d8dc79aeeb55b))

## [0.1.3] - 2021-04-22

### Bug fixes

- Replace current prayer algorithm ([e1c5e4a](https://github.com/azzamsa/islam/commit/e1c5e4a5115498e55a03ad1b83fb7e2156be3210))

  Use more reliable logic.

## [0.0.2] - 2021-04-20

### Features

- Include `chrono` to module ([b5a7afc](https://github.com/azzamsa/islam/commit/b5a7afc1a063a2c75dcc6dd060a6e471cdb270f4))

  So `islam` user doesn't need to install `chrono` separately

### Bug fixes

- Use shorter module name ([b6c3399](https://github.com/azzamsa/islam/commit/b6c3399ebcc6f757d3a3eb5e893637c24f0ecae1))

  It is easier and more beauty for user
