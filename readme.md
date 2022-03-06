# About
Simple Actix (backend) and React (frontend) "proxy" for Unsplash, rather than the frontend directly querying Unsplash, the query will go to the backend, and it will return the results to the frontend. This was put together to supplement my learning of the "Modern React with Redux" Udemy course [(LINK)](https://www.udemy.com/course/react-redux), when building the Pics app. 



https://user-images.githubusercontent.com/9965448/156912076-717f79ee-9035-4250-9e0c-b9d80a1ba178.mp4



# Prerequisites

*   rust (https://rustup.rs/)
*   npm
*   Unsplash API key (https://unsplash.com/developers)

# Setup
Go into frontend directory, install the required npm stuffs

1.  `cd frontend`
2.  `npm install --save-dev react-scripts`

Get an API key from Unsplash and place it into frontend/src/config/config.json

3.  `cd src/config`
4.  `nano config.json` .. "UNSPLASH_API_KEY"

Return to frontend root, Build the frontend

5.  `cd ../..`
6.  `npm run build`

Head back to the root directory, and run the backend

7.  `cd ../`
8.  `cargo run`

9.  From your browser on your local machine, navigate to `localhost:8081`, and run a search.

10. Or, make a curl request against the running server from your terminal:

`curl "http://localhost:8081/img?query=cars" -H "Authorization: Client-ID putyour-api-keyhere"`

