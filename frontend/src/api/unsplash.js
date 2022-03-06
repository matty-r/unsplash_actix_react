import axios from 'axios';
import configData from "../config/config.json"

export default axios.create({
  baseURL: configData.SERVER_URL,
  headers: {
    Authorization: "Client-ID "+configData.UNSPLASH_API_KEY,
  }
});