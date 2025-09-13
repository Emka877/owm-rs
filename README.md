# owm-rs
Simple (unofficial) wrapper to call the [Open Weather Map](https://openweathermap.org/) API.

# Prerequisites
* An OWM API key ([Your Dashboard](https://home.openweathermap.org/api_keys))

# Testing the source code and running examples
Create at the root of the project a `.env` file, with the following content:
```.env
    omw_api_key: "xxxx",
    city_name: "yyyy",
```
Where `xxxx` is your OWM API key (See [Prerequisites](#prerequisites)), and `yyyy` is the city name you want to use for testing. (E.g., `London`)

Then, run the tests with:
`cargo run --color=always --package owm-rs --example basic --profile dev --features=utils`

## Something missing, have a request or something is broken?
Please create an issue [here](https://github.com/Emka877/owm-rs/issues/new) I'll treat it as soon as possible!

# Changelog
## 1.0.17
- Updated dependencies
- Changed the instructions to run tests; now uses a .env file to be created at the root of the project, see [#Testing the source code and running examples](#testing-the-source-code-and-running-examples)
