import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { QRCodeSVG } from "qrcode.react";
import "./App.css";
import AutoStartToggle from "./components/AutoStartToggle";

function App() {
  const [status, setStatus] = useState({ message: "", type: "" });
  const [buttonsDisabled, setButtonsDisabled] = useState(false);
  const [modal, setModal] = useState({ show: false, title: "", message: "", onConfirm: null });
  const [localIp, setLocalIp] = useState("");
  const [volume, setVolume] = useState(null);

  useEffect(() => {
    async function fetchLocalIp() {
      try {
        const ip = await invoke("get_local_ip");
        setLocalIp(ip);
      } catch (error) {
        console.error("Failed to get local IP:", error);
        setLocalIp("Unable to get IP");
      }
    }
    fetchLocalIp();
    fetchVolume();
  }, []);

  async function fetchVolume() {
    console.log("Fetching volume...");
    try {
      const vol = await invoke("get_volume");
      console.log("Volume received from backend:", vol);
      setVolume(vol);
      console.log("Volume state updated to:", vol);
    } catch (error) {
      console.error("Failed to get volume:", error);
    }
  }

  function showModal(title, message) {
    return new Promise((resolve) => {
      setModal({ show: true, title, message, onConfirm: resolve });
    });
  }

  function closeModal(result) {
    if (modal.onConfirm) {
      modal.onConfirm(result);
    }
    setModal({ show: false, title: "", message: "", onConfirm: null });
  }

  async function handleShutdown() {
    const confirmed = await showModal("Confirm Action", "Are you sure you want to shutdown the PC?");

    if (!confirmed) {
      return;
    }

    setButtonsDisabled(true);
    setStatus({ message: "Executing shutdown...", type: "info" });

    try {
      const result = await invoke("shutdown");
      if (result.success) {
        setStatus({ message: "Shutdown command sent successfully!", type: "success" });
      } else {
        setStatus({ message: `Error: ${result.message}`, type: "error" });
        setButtonsDisabled(false);
      }
    } catch (error) {
      setStatus({ message: `Error: ${error}`, type: "error" });
      setButtonsDisabled(false);
    }

    setTimeout(() => setStatus({ message: "", type: "" }), 5000);
  }

  async function handleSleep() {
    const confirmed = await showModal("Confirm Action", "Are you sure you want to put the PC to sleep?");

    if (!confirmed) {
      return;
    }

    setButtonsDisabled(true);
    setStatus({ message: "Executing sleep...", type: "info" });

    try {
      const result = await invoke("sleep");
      if (result.success) {
        setStatus({ message: "Sleep command sent successfully!", type: "success" });
      } else {
        setStatus({ message: `Error: ${result.message}`, type: "error" });
        setButtonsDisabled(false);
      }
    } catch (error) {
      setStatus({ message: `Error: ${error}`, type: "error" });
      setButtonsDisabled(false);
    }

    setTimeout(() => setStatus({ message: "", type: "" }), 5000);
  }

  async function handleCancel() {
    setStatus({ message: "Cancelling shutdown...", type: "info" });

    try {
      const result = await invoke("cancel_shutdown");
      if (result.success) {
        setStatus({ message: result.message, type: "success" });
      } else {
        setStatus({ message: `Error: ${result.message}`, type: "error" });
      }
    } catch (error) {
      setStatus({ message: `Error: ${error}`, type: "error" });
    }

    setTimeout(() => setStatus({ message: "", type: "" }), 5000);
  }

  async function handleVolumeIncrease() {
    console.log("Volume increase button clicked");
    try {
      await invoke("increase_volume", { amount: 2 });
      console.log("Volume increased, fetching new volume...");
      // Wait a bit for the volume change to take effect
      setTimeout(fetchVolume, 200);
    } catch (error) {
      console.error("Failed to increase volume:", error);
    }
  }

  async function handleVolumeDecrease() {
    console.log("Volume decrease button clicked");
    try {
      await invoke("decrease_volume", { amount: 2 });
      console.log("Volume decreased, fetching new volume...");
      // Wait a bit for the volume change to take effect
      setTimeout(fetchVolume, 200);
    } catch (error) {
      console.error("Failed to decrease volume:", error);
    }
  }

  return (
    <main className="container">
      <AutoStartToggle />
      <h1>Ferrous Control</h1>
      <p className="subtitle">Remote PC Control Panel</p>
      {localIp && (
        <div className="network-info">
          <p className="ip-address">Network Address: http://{localIp}:7777</p>
          <div className="qr-code-container">
            <QRCodeSVG
              value={`http://${localIp}:7777`}
              size={180}
              level="H"
            />
            <p className="qr-label">Scan to connect from mobile device</p>
          </div>
        </div>
      )}

      <div className="button-container">
        <button
          className="control-btn shutdown-btn"
          onClick={handleShutdown}
          disabled={buttonsDisabled}
        >
          <span className="icon">🔴</span>
          <span>Shutdown</span>
        </button>

        <button
          className="control-btn sleep-btn"
          onClick={handleSleep}
          disabled={buttonsDisabled}
        >
          <span className="icon">😴</span>
          <span>Sleep</span>
        </button>

        <button
          className="control-btn cancel-btn"
          onClick={handleCancel}
        >
          <span className="icon">⛔</span>
          <span>Cancel</span>
        </button>
      </div>

      <div className="button-container volume-controls">
        <button
          className="control-btn volume-up-btn"
          onClick={handleVolumeIncrease}
        >
          <span className="icon">🔊</span>
          <span>Volume Up</span>
        </button>

        <button
          className="control-btn volume-down-btn"
          onClick={handleVolumeDecrease}
        >
          <span className="icon">🔉</span>
          <span>Volume Down</span>
        </button>
      </div>

      {volume !== null && (
        <div className="volume-display">
          <div className="volume-level">
            <span className="volume-icon">🔊</span>
            <span className="volume-percentage">{volume}%</span>
          </div>
          <div className="volume-bar">
            <div className="volume-bar-fill" style={{ width: `${volume}%` }}></div>
          </div>
        </div>
      )}

      {status.message && (
        <div className={`status ${status.type}`}>
          {status.message}
        </div>
      )}

      {modal.show && (
        <div className="modal-overlay active" onClick={(e) => {
          if (e.target.className.includes('modal-overlay')) {
            closeModal(false);
          }
        }}>
          <div className="modal">
            <h2>{modal.title}</h2>
            <p>{modal.message}</p>
            <div className="modal-buttons">
              <button className="modal-btn modal-btn-cancel" onClick={() => closeModal(false)}>
                Cancel
              </button>
              <button className="modal-btn modal-btn-confirm" onClick={() => closeModal(true)}>
                Confirm
              </button>
            </div>
          </div>
        </div>
      )}
    </main>
  );
}

export default App;
