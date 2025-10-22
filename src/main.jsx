import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import { InitTray } from "./utils/Tray";

InitTray();

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
