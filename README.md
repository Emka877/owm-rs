# owm-rs
Simple (unofficial) wrapper to call the [Open Weather Map](https://openweathermap.org/) API.

# Prerequisites
* An OWM API key ([Your Dashboard](https://home.openweathermap.org/api_keys))

# Testing the source code and running examples
If you want to run tests, create the `test_data` folder at the root of the project.

Then create `test_data/credentials.ron`:
```ron
Credentials (
    omw_api_key: "xxxx",
    city_name: "yyyy",
)
```