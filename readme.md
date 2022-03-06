# Setup
Go into frontend directory, install the required npm stuffs

1.  `cd frontend`
2.  `npm install --save-dev react-scripts`

Get an API key from Unsplash and place it into frontend/src/config/config.json

1.  `nano config.json` .. "UNSPLASH_API_KEY"

Build the frontend

1.  `npm run build`

Head back to the root directory, and run the backend

1.  `cd ../`
2.  `cargo run`

Log in to `localhost:8081` on your local machine, and run a search