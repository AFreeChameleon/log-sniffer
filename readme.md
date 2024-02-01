# Hog

A program that reads the output of your commands and sends them to a server to back them up. See the website [here](https://hog.chameleo.dev/).

Create an api key and then go to: `https://hog.chameleo.dev/logs/<YOUR_API_KEY>`

Install hog onto your computer.

NOTE: no installer on mac so you will have to build from source.

Then run the command you want to monitor providing the api key using this syntax:

```
hog --command "npm start" --key <YOUR_API_KEY>
```
