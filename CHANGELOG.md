# Changelog

All notable changes to this project will be documented in this file.

## [2.0.0] - 2023-06-22

### Bug fixes

- Restructure ([eb33501](eb33501733580178938e689a3147983eb58e5b0a))
- Use chrono v0.4.26 ([b57aa45](b57aa45e1f6855d25c082cb1273a95ab06f3c633))
- Migrate back to chrono ([bc53fa5](bc53fa59123c0ae0ebea5e16f8be0655c8e821f6))

## [1.0.0] - 2023-01-22

### Bug fixes

- User doesn't need to pass UTC offset anymore ([e29c32e](e29c32e2bbea3b7eb62cf134ba58a247f851b8c5))
  - **BREAKING!** ⚠️ : `time` has a feature to get the UTC offset from the user's
    system. Passing the offset to the app is redundant.

## [0.1.5] - 2022-05-18

### Bug fixes

- They are not executables ([31bf1fd](31bf1fd982fdf4aae30e1a94dd1d8dc79aeeb55b))

## [0.1.3] - 2021-04-22

### Bug fixes

- Replace current prayer algorithm ([e1c5e4a](e1c5e4a5115498e55a03ad1b83fb7e2156be3210))

## [0.0.2] - 2021-04-20

### Features

- Include `chrono` to module ([b5a7afc](b5a7afc1a063a2c75dcc6dd060a6e471cdb270f4))

### Bug fixes

- Use shorter module name ([b6c3399](b6c3399ebcc6f757d3a3eb5e893637c24f0ecae1))
